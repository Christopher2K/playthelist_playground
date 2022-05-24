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
