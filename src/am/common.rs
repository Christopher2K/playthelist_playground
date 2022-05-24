use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct AppleApiObject<T> {
    pub id: String,
    pub href: String,
    pub attributes: T,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AppleApiObjectWithRelationship<T, R> {
    pub id: String,
    pub href: String,
    pub attributes: T,
    pub relationships: R,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AppleCollectionResponse<T> {
    pub data: Vec<T>,
    pub next: Option<String>,
}
