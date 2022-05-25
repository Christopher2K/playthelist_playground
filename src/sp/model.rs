use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ExternalIds {
    pub isrc: String,
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
pub struct Track {
    pub id: String,
    pub name: String,
    pub uri: String,
    pub album: TrackAlbum,
    pub external_ids: ExternalIds,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SearchTrackItems {
    pub items: Vec<Track>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserPlaylist {
    pub collaborative: bool,
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlaylistTrackItem {
    pub track: Track,
}
