use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct SpotifyError {
    status: u16,
    message: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SpotifyErrorResponse {
    error: SpotifyError,
}
