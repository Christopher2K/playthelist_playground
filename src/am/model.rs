use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SongAttribute {
    pub isrc: String,
    pub album_name: String,
    pub artist_name: String,
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LibraryPlaylistAttribute {
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StorefrontAttributes {
    pub default_language_tag: String,
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TrackArtwork {
    pub width: u16,
    pub height: u16,
    pub url: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LibrarySongAttribute {
    pub name: String,
    pub artist_name: String,
    pub album_name: Option<String>,
    pub artwork: TrackArtwork,
}
