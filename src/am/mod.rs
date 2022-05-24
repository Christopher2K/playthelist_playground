use common::*;
use model::*;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderName, AUTHORIZATION};
use response::*;
use std::collections::HashMap;
use std::str::FromStr;

pub mod common;
pub mod constant;
pub mod domain;
pub mod model;
pub mod relationship;
pub mod response;

pub struct AppleMusicAPI {
    user_token: String,
    dev_token: String,
}

impl AppleMusicAPI {
    const BASE_URL: &'static str = "https://api.music.apple.com/v1";
    const UNVERSIONED_BASE_URL: &'static str = "https://api.music.apple.com";

    pub fn new(user_token: &str, dev_token: &str) -> Self {
        Self {
            user_token: String::from(user_token),
            dev_token: String::from(dev_token),
        }
    }

    pub fn get_user_storefront(&self) -> Result<StorefrontResponse, reqwest::Error> {
        let url = format!("{}/me/storefront", Self::BASE_URL);
        let client = Client::new();

        let result = client.get(url).headers(self.get_base_headers()).send();

        result.and_then(|response| response.json::<StorefrontResponse>())
    }

    pub fn get_all_playlists(&self) -> Result<LibraryPlaylistsResponse, reqwest::Error> {
        let url = format!("{}/me/library/playlists", Self::BASE_URL);
        let client = Client::new();

        let result = client.get(url).headers(self.get_base_headers()).send();

        result.and_then(|response| response.json::<LibraryPlaylistsResponse>())
    }

    pub fn get_user_playlist_tracks(
        &self,
        playlist: &AppleApiObject<LibraryPlaylistAttribute>,
        mut responses: Vec<LibrarySongsResponse>,
        next: Option<String>,
    ) -> Result<Vec<LibrarySongsResponse>, reqwest::Error> {
        let url = match next {
            Some(next_url) => format!(
                "{}/{}&include=catalog",
                Self::UNVERSIONED_BASE_URL,
                next_url
            ),
            None => format!(
                "{}/me/library/playlists/{}/tracks?include=catalog",
                Self::BASE_URL,
                &playlist.id
            ),
        };

        let client = Client::new();
        let result = client.get(url).headers(self.get_base_headers()).send();

        result
            .and_then(|response| response.json::<LibrarySongsResponse>())
            .and_then(|response| {
                let response_next = response.next.clone();
                responses.push(response);

                match response_next {
                    Some(next_url) => self.get_user_playlist_tracks(
                        playlist,
                        responses,
                        Some(String::from(next_url)),
                    ),
                    None => Ok(responses),
                }
            })
    }

    // PRIVATE STUFFS
    fn get_base_headers(&self) -> HeaderMap {
        let headers = HashMap::from([
            (
                AUTHORIZATION.to_string(),
                String::from(format!("Bearer {}", &self.dev_token)),
            ),
            (
                HeaderName::from_str("Music-User-Token")
                    .expect("Cannot parse Music-User-Token as Header")
                    .to_string(),
                self.user_token.clone(),
            ),
        ]);

        (&headers)
            .try_into()
            .expect("Cannot convert hashmap to headermap")
    }
}
