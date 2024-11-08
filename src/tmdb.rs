use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

use crate::TVSeriesOption;

const TOKEN: &str = "";

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
        .bearer_auth(TOKEN)
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
        .bearer_auth(TOKEN)
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
        .bearer_auth(TOKEN)
        .header("accept", "application/json")
        .send()
        .unwrap()
        .json()
        .unwrap();

    let mut options = vec![];
    for (i, show) in response.results.iter().enumerate().take(5) {
        options.push(TVSeriesOption {
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
