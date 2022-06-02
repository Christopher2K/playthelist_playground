use crate::am::common::AppleRequest;
use crate::am::model::*;

// REQUEST OBJECTS
pub type LibraryPlaylistCreationRequest =
    AppleRequest<LibraryPlaylistCreationAttributes, LibraryPlaylistCreationRelationships>;

pub type LibraryPlaylistTracksRequest = LibraryPlaylistCreationTracks;
