use rand::prelude::*;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::io;

const TOKEN: &str = "";

#[derive(Debug, Serialize, Deserialize)]
struct TelevisionEpisodeResponse {
    episode_number: i32,
    name: String,
    overview: String,
    runtime: i32,
    season_number: i32,
    id: i32,
}

struct TelevisionEpisode {
    show_name: String,
    season: i32,
    title: String,
    number: i32,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ShowInfoResponse {
    page: i32,
    results: Vec<ShowInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TVSeriesDetailsResponse {
    number_of_episodes: i32,
    number_of_seasons: i32,
    seasons: Vec<TVSeriesDetailsSeason>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TVSeriesDetailsSeason {
    id: i32,
    season_number: i32,
    episode_count: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct ShowInfo {
    id: i32,
    name: String,
    origin_country: Vec<String>,
}

#[derive(Debug)]
struct ShowOption {
    number: usize,
    label: String,
    value: i32,
}

fn main() {
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

    // let show_number = show_number.trim();
    println!(
        "Getting episode data for '{}'",
        options[show_number.trim().parse::<usize>().unwrap() - 1].label
    );
    println!("");

    let season_data = fetch_season_data(
        &client,
        &options[show_number.trim().parse::<usize>().unwrap() - 1].value,
    );

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
        &options[show_number.trim().parse::<usize>().unwrap() - 1].value,
        random_episode.0,
        random_episode.1,
    );

    let episode = TelevisionEpisode {
        show_name: options.first().unwrap().label.clone(),
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

fn fetch_show_info(client: &Client, query: &str) -> Vec<ShowOption> {
    let url = format!(
        "https://api.themoviedb.org/3/search/tv?query={}&include_adult=false&language=en-US&page=1",
        query
    );

    let response: ShowInfoResponse = client
        .get(&url)
        .bearer_auth(TOKEN)
        .header("accept", "application/json")
        .send()
        .unwrap()
        .json()
        .unwrap();

    let mut options = vec![];
    for (i, show) in response.results.iter().enumerate().take(5) {
        options.push(ShowOption {
            number: i + 1,
            // TODO: Can I only show country if multiple share the same name?
            // XXX: Also happens with shows from same country but different year
            // EG: Married with Children
            label: format!("{} ({})", show.name.clone(), show.origin_country.join(", ")),
            value: show.id,
        })
    }

    return options;
}

fn fetch_season_data(client: &Client, id: &i32) -> TVSeriesDetailsResponse {
    let url = format!("https://api.themoviedb.org/3/tv/{}?language=en-US", id);

    let response: TVSeriesDetailsResponse = client
        .get(&url)
        .bearer_auth(TOKEN)
        .header("accept", "application/json")
        .send()
        .unwrap()
        .json()
        .unwrap();

    // println!("{:?}", response);
    return response;
}

fn fetch_episode(
    client: &Client,
    id: &i32,
    season: i32,
    episode: i32,
) -> TelevisionEpisodeResponse {
    let url = format!(
        "https://api.themoviedb.org/3/tv/{}/season/{}/episode/{}?language=en-US",
        id, season, episode
    );

    let response: TelevisionEpisodeResponse = client
        .get(&url)
        .bearer_auth(TOKEN)
        .header("accept", "application/json")
        .send()
        .unwrap()
        .json()
        .unwrap();

    return response;
}

fn print_episode(episode: TelevisionEpisode) {
    println!("{}", episode.show_name);
    println!("Season {}", episode.season);
    println!("Episode {} - {}", episode.number, episode.title);
    println!("");
    println!("{}", episode.description);
}
