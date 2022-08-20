use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Oauth2Token {
    pub token_type: String,
    pub expires_in: usize,
    pub access_token: String,
    pub scope: String,
    pub refresh_token: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TweetResponse {
    pub data: TweetResponseInner,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TweetResponseInner {
    pub id: String,
    pub text: String,
}
