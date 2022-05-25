use serde::{Deserialize, Serialize};

use crate::sp::model::*;

#[derive(Deserialize, Serialize, Debug)]
pub struct SpotifySearchTrackResponse {
    pub tracks: SearchTrackItems,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SpotifyPlaylistResponse {
    pub id: String,
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SpotifyUserResponse {
    pub id: String,
    // pub country: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SpotifyUserPlaylistsResponse {
    pub items: Vec<UserPlaylist>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SpotifyPlaylistTracksResponse {
    pub items: Vec<PlaylistTrackItem>,
    pub next: Option<String>,
}
