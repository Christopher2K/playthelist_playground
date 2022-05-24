use crate::am::common::{AppleApiObject, AppleCollectionResponse};
use crate::am::model::SongAttribute;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct CatalogRelationship {
    pub catalog: AppleCollectionResponse<AppleApiObject<SongAttribute>>,
}
