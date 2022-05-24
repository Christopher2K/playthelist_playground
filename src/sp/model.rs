use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ExternalIds {
    isrc: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TrackAlbum {
    pub album_type: AlbumType,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum AlbumType {
    #[serde(rename = "single")]
    Single,
    #[serde(rename = "album")]
    Album,
    #[serde(rename = "compilation")]
    Compilation,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct SearchTrackItem {
    pub id: String,
    pub name: String,
    pub uri: String,
    pub album: TrackAlbum,
    pub external_ids: ExternalIds,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SearchTrackItems {
    pub items: Vec<SearchTrackItem>,
}

//
//
// RESPONSES
//
//
