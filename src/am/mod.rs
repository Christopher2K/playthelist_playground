use common::*;
use model::*;
use request::*;
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
pub mod request;
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

    pub fn search_track(
        &self,
        storefront_name: &str,
        isrc: &str,
    ) -> Result<SongsResponse, reqwest::Error> {
        let params = format!("filter[isrc]={}", isrc);
        let url = format!(
            "{}/catalog/{}/songs?{}",
            Self::BASE_URL,
            storefront_name,
            params
        );

        let client = Client::new();
        let headers = self.get_base_headers();

        client
            .get(url)
            .headers(headers)
            .send()
            .and_then(|response| response.json())
    }

    pub fn create_new_playlist(
        &self,
        name: &str,
        tracks_id: Vec<String>,
    ) -> Result<Vec<reqwest::blocking::Response>, reqwest::Error> {
        let url = format!("{}/me/library/playlists", Self::BASE_URL);
        let headers = self.get_base_headers();
        let (first_track, other_tracks) = tracks_id.split_first().unwrap();

        let body = LibraryPlaylistCreationRequest {
            attributes: LibraryPlaylistCreationAttributes {
                name: String::from(name),
            },
            relationships: Some(LibraryPlaylistCreationRelationships {
                tracks: LibraryPlaylistCreationTracks {
                    data: vec![LibraryPlaylistCreationTrack {
                        id: String::from(first_track),
                        track_type: LibraryTrackType::Songs,
                    }],
                },
            }),
        };

        let client = Client::new();
        client
            .post(url)
            .headers(headers)
            .json(&body)
            .send()
            .and_then(|response| response.json::<LibraryPlaylistsResponse>())
            .and_then(|p| {
                let new_playlist = &p.data[0];
                self.add_tracks_to_playlist(&new_playlist.id, other_tracks)
            })
    }

    fn add_tracks_to_playlist(
        &self,
        playlist_id: &str,
        tracks: &[String],
    ) -> Result<Vec<reqwest::blocking::Response>, reqwest::Error> {
        let url = format!(
            "{}/me/library/playlists/{}/tracks",
            Self::BASE_URL,
            playlist_id
        );
        let headers = self.get_base_headers();
        let client = Client::new();

        // Split isrc list -> Want to have 100 items per chunks
        tracks
            .chunks(100)
            .map(|tracks_chunk| {
                let data = LibraryPlaylistCreationTracks {
                    data: tracks_chunk
                        .iter()
                        .map(|id| LibraryPlaylistCreationTrack {
                            id: String::from(id),
                            track_type: LibraryTrackType::Songs,
                        })
                        .collect(),
                };

                println!("{}", serde_json::to_string_pretty(&data).unwrap());

                client
                    .post(&url)
                    .headers(headers.clone())
                    .json(&data)
                    .send()
            })
            .collect::<_>()
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
