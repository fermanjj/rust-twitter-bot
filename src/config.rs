use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub bucket_name: String,
    pub code: String,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    #[serde(with = "humantime_serde")]
    pub request_timeout: Duration,
}
