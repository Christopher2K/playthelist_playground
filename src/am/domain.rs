use crate::am::common::AppleApiObjectWithRelationship;
use crate::am::model::LibrarySongAttribute;
use crate::am::relationship::CatalogRelationship;

use serde::{Deserialize, Serialize};

type AppleMusicLibraryPlaylistObject =
    AppleApiObjectWithRelationship<LibrarySongAttribute, CatalogRelationship>;

#[derive(Deserialize, Serialize, Debug)]
pub struct AMPlaylistTrack {
    pub library_song_id: String,
    pub name: String,
    pub artist_name: String,
    pub album_name: Option<String>,
    pub isrc: Option<String>,
    pub catalog_id: Option<String>,
}

impl AMPlaylistTrack {
    pub fn to_playlist_track(apple_object: &AppleMusicLibraryPlaylistObject) -> Self {
        let mut base_object = AMPlaylistTrack {
            library_song_id: apple_object.id.clone(),
            artist_name: apple_object.attributes.artist_name.clone(),
            name: apple_object.attributes.name.clone(),
            album_name: None,
            catalog_id: None,
            isrc: None,
        };

        if let Some(album_name) = apple_object.attributes.album_name.clone() {
            base_object.album_name = Some(album_name);
        }

        if let Some(catalog) = apple_object.relationships.catalog.data.first() {
            base_object.catalog_id = Some(catalog.id.clone());
            base_object.isrc = Some(catalog.attributes.isrc.clone());
        }

        base_object
    }
}
