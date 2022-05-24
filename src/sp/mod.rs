use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};
use reqwest::StatusCode;
use std::collections::HashMap;
use urlencoding::encode;

use common::SpotifyErrorResponse;
use response::*;

pub mod common;
pub mod constant;
pub mod model;
pub mod response;

pub struct SearchTrackInfo {
    pub name: String,
    pub album_name: String,
    pub artist_name: String,
}

pub type SearchTrackISrc = Option<String>;

pub struct SearchTrackArgument {
    pub isrc: SearchTrackISrc,
    pub track_info: SearchTrackInfo,
}

pub struct SpotifyAPI {
    user_token: String,
}

impl SpotifyAPI {
    const BASE_URL: &'static str = "https://api.spotify.com/v1";

    pub fn new(user_token: &str) -> Self {
        Self {
            user_token: String::from(user_token),
        }
    }

    fn get_base_headers(&self) -> HeaderMap {
        let headers = HashMap::from([
            (
                AUTHORIZATION.to_string(),
                String::from(format!("Bearer {}", &self.user_token)),
            ),
            (CONTENT_TYPE.to_string(), String::from("application/json")),
        ]);

        (&headers)
            .try_into()
            .expect("Cannot convert hashmap to headermap")
    }

    pub fn get_user_profile(&self) -> Result<SpotifyUserResponse, reqwest::Error> {
        let url = format!("{}/me", Self::BASE_URL);
        let client = Client::new();
        let headers = self.get_base_headers();

        client
            .get(url)
            .headers(headers)
            .send()
            .and_then(|response| response.json::<SpotifyUserResponse>())
    }

    pub fn search_track(
        &self,
        search_args: SearchTrackArgument,
    ) -> Result<Result<SpotifySearchTrackResponse, SpotifyErrorResponse>, reqwest::Error> {
        let query = match search_args.isrc {
            Some(x) => format!("isrc:{}", x),
            _ => {
                let parameters = HashMap::from([
                    ("artist", &search_args.track_info.artist_name),
                    ("album", &search_args.track_info.album_name),
                    ("track", &search_args.track_info.name),
                ]);

                parameters
                    .iter()
                    .map(|(k, v)| format!("{}:{}", &k, encode(&v)))
                    .collect::<Vec<_>>()
                    .join("+")
            }
        };

        let url = format!("{}/search?type=track&q={}", Self::BASE_URL, query);
        let headers = self.get_base_headers();

        let client = Client::new();
        client
            .get(url)
            .headers(headers)
            .send()
            .map(|response| match response.status() {
                StatusCode::OK => Ok(response.json::<SpotifySearchTrackResponse>().unwrap()),
                _ => Err(response.json::<SpotifyErrorResponse>().unwrap()),
            })
    }

    pub fn create_new_playlist(
        &self,
        user_id: &str,
        name: &str,
    ) -> Result<SpotifyPlaylistResponse, reqwest::Error> {
        let url = format!("{}/users/{}/playlists", Self::BASE_URL, user_id);
        let client = Client::new();
        let headers = self.get_base_headers();

        let data = HashMap::from([("name", name)]);
        client
            .post(url)
            .headers(headers)
            .json(&data)
            .send()
            .and_then(|response| response.json::<SpotifyPlaylistResponse>())
    }

    pub fn add_tracks_to_playlist(
        &self,
        playlist_id: &str,
        tracks: &[String],
    ) -> Result<Vec<reqwest::blocking::Response>, reqwest::Error> {
        let url = format!("{}/playlists/{}/tracks", Self::BASE_URL, playlist_id);
        let client = Client::new();

        tracks
            .chunks(100)
            .map(|tracks_chunk| {
                let headers = self.get_base_headers();
                let data = HashMap::from([("uris", tracks_chunk)]);
                client.post(&url).headers(headers).json(&data).send()
            })
            .collect::<_>()
    }
}
