use crate::am::common::*;
use crate::am::model::*;
use crate::am::relationship::*;

pub type StorefrontResponse = AppleCollectionResponse<AppleApiObject<StorefrontAttributes>>;
pub type LibraryPlaylistsResponse =
    AppleCollectionResponse<AppleApiObject<LibraryPlaylistAttribute>>;
pub type LibrarySongsResponse = AppleCollectionResponse<
    AppleApiObjectWithRelationship<LibrarySongAttribute, CatalogRelationship>,
>;
