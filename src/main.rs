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
    // println!("");
    // println!("");

    fetch_show_info(client, &query);

    println!("");
    println!("Enter the number of the show you want to watch");
    let mut show_number = String::new();
    let _show = io::stdin()
        .read_line(&mut show_number)
        .expect("Failed to read line");
    println!("");

    // println!("Finding an episode...");

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

fn fetch_show_info(client: Client, query: &str) {
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

    println!("Found {} show(s) for '{}", response.results.len(), query);

    for (i, show) in response.results.iter().enumerate().take(5) {
        println!("{}. {}", i + 1, show.name);
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct ShowInfoResponse {
    page: i32,
    results: Vec<ShowInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ShowInfo {
    name: String,
}

fn print_episode(episode: TelevisionEpisode) {
    println!("{}", episode.show_name);
    println!("Season {}", episode.season);
    println!("Episode {} - {}", episode.number, episode.title);
    println!("");
    println!("{}", episode.description);
}
