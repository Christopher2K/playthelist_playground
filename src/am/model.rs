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

#[derive(Serialize, Deserialize, Debug)]
pub struct LibraryPlaylistCreationAttributes {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LibraryPlaylistCreationRelationships {
    pub tracks: LibraryPlaylistCreationTracks,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LibraryPlaylistCreationTracks {
    pub data: Vec<LibraryPlaylistCreationTrack>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LibraryPlaylistCreationTrack {
    pub id: String,
    #[serde(rename = "type")]
    pub track_type: LibraryTrackType,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum LibraryTrackType {
    #[serde(rename = "library-music-videos")]
    LibraryMusicVideos,
    #[serde(rename = "library-songs")]
    LibrarySongs,
    #[serde(rename = "music-videos")]
    MusicVideos,
    #[serde(rename = "songs")]
    Songs,
}
