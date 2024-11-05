// use colored::*;
use reqwest::blocking::Client;
// use reqwest::Error;
use serde::{Deserialize, Serialize};
// use std::env;
use std::io;

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
}

#[derive(Debug)]
struct ShowOption {
    number: usize,
    label: String,
    value: i32,
}

fn main() {
    println!("Welcome to rand-watch");
    println!("Type in a show name to get a random episode");

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
    let mut show_number = String::new();
    let _show = io::stdin()
        .read_line(&mut show_number)
        .expect("Failed to read line");
    println!("");

    println!("Finding an episode...");
    println!("");

    let episodes = fetch_more_data(&client, &options.first().unwrap().value);

    let episode = TelevisionEpisode {
        show_name: String::from("The Office (US)"),
        season: 6,
        title: String::from("Scott's Tots"),
        number: 12,
        description: String::from("Two new, young employees throw Dwight and Jim off balance; Andy returns from leadership training."),
    };

    print_episode(episode);

    println!("");
    println!("Enjoy the show!");
}

fn fetch_show_info(client: &Client, query: &str) -> Vec<ShowOption> {
    let token = "";

    let url = format!(
        "https://api.themoviedb.org/3/search/tv?query={}&include_adult=false&language=en-US&page=1",
        query
    );

    let response: ShowInfoResponse = client
        .get(&url)
        .bearer_auth(token)
        .header("accept", "application/json")
        .send()
        .unwrap()
        .json()
        .unwrap();

    let mut options = vec![];
    for (i, show) in response.results.iter().enumerate().take(5) {
        options.push(ShowOption {
            number: i + 1,
            label: show.name.clone(),
            value: show.id,
        })
    }

    return options;
}

fn fetch_more_data(client: &Client, id: &i32) {
    let token = "";

    let url = format!("https://api.themoviedb.org/3/tv/{}?language=en-US", id);

    let response: TVSeriesDetailsResponse = client
        .get(&url)
        .bearer_auth(token)
        .header("accept", "application/json")
        .send()
        .unwrap()
        .json()
        .unwrap();

    println!("{:?}", response);
}

fn print_episode(episode: TelevisionEpisode) {
    println!("{}", episode.show_name);
    println!("Season {}", episode.season);
    println!("Episode {} - {}", episode.number, episode.title);
    println!("");
    println!("{}", episode.description);
}
