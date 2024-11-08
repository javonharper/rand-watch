use rand::prelude::*;
use std::io;
use tmdb::{fetch_episode, fetch_season_data, fetch_show_info};

mod tmdb;

struct TVEpisodeOutput {
    show_name: String,
    season: i32,
    title: String,
    number: i32,
    description: String,
}

#[derive(Debug)]
struct TVSeriesOption {
    number: usize,
    label: String,
    value: i32,
}

fn main() {
    println!("");
    println!("");

    println!("Welcome to rand-watch");
    println!("Enter a show name to get a random episode");

    println!("");
    let mut query = String::new();
    let _show = io::stdin()
        .read_line(&mut query)
        .expect("Failed to read line");
    println!("");

    let client = reqwest::blocking::Client::new();
    let options = fetch_show_info(&client, &query);

    println!("Found {} show(s) for '{}'", options.len(), query.trim());
    for option in options.iter() {
        println!("{}. {}", option.number, option.label);
    }

    println!("");
    println!("Enter the number of the show you want to watch");
    println!("");
    let mut show_number = String::new();
    io::stdin()
        .read_line(&mut show_number)
        .expect("Failed to read line");
    println!("");

    let option_value = show_number.trim().parse::<usize>().unwrap();

    println!(
        "Getting episode data for '{}'",
        options[option_value - 1].label
    );
    println!("");

    let season_data = fetch_season_data(&client, &options[option_value - 1].value);

    let mut episodes = vec![];
    for season in season_data.seasons.iter() {
        for i in 1..=season.episode_count {
            // Omit season 0
            if season.season_number == 0 {
                continue;
            }
            episodes.push((season.season_number, i));
        }
    }

    let random_episode = episodes.choose(&mut rand::thread_rng()).unwrap();
    let episode_response = fetch_episode(
        &client,
        &options[option_value - 1].value,
        random_episode.0,
        random_episode.1,
    );

    let episode = TVEpisodeOutput {
        show_name: options[option_value - 1].label.clone(),
        season: episode_response.season_number,
        title: episode_response.name,
        number: episode_response.episode_number,
        description: episode_response.overview,
    };

    print_episode(episode);

    println!("");
    println!("Enjoy the show!");
    println!("");
}

fn print_episode(episode: TVEpisodeOutput) {
    println!("{}", episode.show_name);
    println!("Season {}", episode.season);
    println!("Episode {} - {}", episode.number, episode.title);
    println!("");
    println!("{}", episode.description);
}
