use std::collections::HashMap;

use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

use crate::TVSeriesOption;

fn get_token() -> String {
    let token = std::env::var("TOKEN").expect("TOKEN must be set.");
    return token;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TVSeriesDetailsSeason {
    pub id: i32,
    pub season_number: i32,
    pub episode_count: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShowInfo {
    pub id: i32,
    pub name: String,
    pub origin_country: Vec<String>,
    pub first_air_date: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShowInfoResponse {
    pub page: i32,
    pub results: Vec<ShowInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TVSeriesDetailsResponse {
    pub number_of_episodes: i32,
    pub number_of_seasons: i32,
    pub seasons: Vec<TVSeriesDetailsSeason>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TelevisionEpisodeResponse {
    pub episode_number: i32,
    pub name: String,
    pub overview: String,
    pub runtime: i32,
    pub season_number: i32,
    pub id: i32,
}

pub fn fetch_episode(
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
        .bearer_auth(get_token())
        .header("accept", "application/json")
        .send()
        .unwrap()
        .json()
        .unwrap();

    return response;
}

pub fn fetch_season_data(client: &Client, id: &i32) -> TVSeriesDetailsResponse {
    let url = format!("https://api.themoviedb.org/3/tv/{}?language=en-US", id);

    let response: TVSeriesDetailsResponse = client
        .get(&url)
        .bearer_auth(get_token())
        .header("accept", "application/json")
        .send()
        .unwrap()
        .json()
        .unwrap();

    // println!("{:?}", response);
    return response;
}

pub fn fetch_show_info(client: &Client, query: &str) -> Vec<TVSeriesOption> {
    let url = format!(
        "https://api.themoviedb.org/3/search/tv?query={}&include_adult=false&language=en-US&page=1",
        query
    );

    let response: ShowInfoResponse = client
        .get(&url)
        .bearer_auth(get_token())
        .header("accept", "application/json")
        .send()
        .unwrap()
        .json()
        .unwrap();

    let mut options = vec![];

    // Get the count of each show name
    let mut show_name_counts: HashMap<String, usize> = HashMap::new();
    for (_, show) in response.results.iter().enumerate().take(5) {
        let count = show_name_counts.entry(show.name.clone()).or_insert(0);
        *count += 1;
    }

    for (i, show) in response.results.iter().enumerate().take(5) {
        let basic_label = format!(
            "{} ({})",
            show.name.clone(),
            show.first_air_date.split("-").next().unwrap(),
        );

        let label_with_country = format!(
            "{} ({}) [{}]",
            show.name.clone(),
            show.first_air_date.split("-").next().unwrap(),
            show.origin_country.join(", ")
        );

        let label;
        if show_name_counts.get(&show.name.clone()).unwrap() > &1 {
            // If there are multiple shows with the same name, use the one with the country
            label = label_with_country;
        } else {
            // Otherwise, use the basic label
            label = basic_label;
        }

        options.push(TVSeriesOption {
            number: i + 1,

            label: label,
            value: show.id,
        })
    }

    return options;
}
