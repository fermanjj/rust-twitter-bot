mod config;
mod model;

use crate::config::Config;
use crate::model::{Oauth2Token, TweetResponse};
use aws_sdk_s3::output::{GetObjectOutput, PutObjectOutput};
use aws_sdk_s3::types::ByteStream;
use aws_sdk_s3::Client;
use aws_smithy_http::body::SdkBody;
use reqwest::Method;
use std::collections::HashMap;

const S3_FILE_KEY: &str = "oauth.json";

#[tokio::main]
async fn main() {
    // load .env file
    dotenv::dotenv().unwrap_or_default();

    let config = envy::from_env::<Config>().expect("Failed to load ENV vars");

    let shared_aws_config = aws_config::load_from_env().await;
    let s3_client = Client::new(&shared_aws_config);

    let token_file = get_token_from_s3(&s3_client, &config).await;

    let oauth_token: Oauth2Token = serde_json::from_str(
        std::str::from_utf8(
            &token_file
                .body
                .collect()
                .await
                .map(|data| data.into_bytes())
                .expect("Failed to get s3 body as bytes"),
        )
        .expect("Failed to read bytes as &str"),
    )
    .expect("Failed to parse file into Oauth2Token json");

    let new_oauth_token = make_refresh_token_request(&config, &oauth_token.refresh_token).await;

    store_token_in_s3(
        &s3_client,
        &config,
        serde_json::to_string_pretty(&new_oauth_token)
            .expect("Failed to convert new_oauth_token to json")
            .as_str(),
    )
    .await;

    let tweet = send_tweet(config, &new_oauth_token, "Fifth test").await;

    println!("tweet_id: {}", tweet.data.id);
}

fn get_reqwest_client(config: &Config) -> reqwest::Client {
    reqwest::Client::builder()
        .timeout(config.request_timeout)
        .build()
        .expect("Failed to build client")
}

async fn send_tweet(config: Config, oauth: &Oauth2Token, text: &str) -> TweetResponse {
    let client = get_reqwest_client(&config);

    client
        .request(Method::POST, "https://api.twitter.com/2/tweets")
        .bearer_auth(&oauth.access_token)
        .json(&HashMap::from([("text", text)]))
        .send()
        .await
        .expect("Failed to send tweet")
        .json()
        .await
        .expect("Failed to get text")
}

#[allow(dead_code)]
async fn make_access_token_request(config: Config) -> Oauth2Token {
    let client = get_reqwest_client(&config);

    let params = [
        ("code", config.code.as_str()),
        ("grant_type", "authorization_code"),
        ("redirect_uri", &config.redirect_uri),
        ("code_verifier", "challenge"),
    ];

    client
        .post("https://api.twitter.com/2/oauth2/token")
        .basic_auth(&config.client_id, Some(&config.client_secret))
        .header("content-type", "application/x-www-form-urlencoded")
        .form(&params)
        .send()
        .await
        .expect("Request failed")
        .json()
        .await
        .expect("Failed to get response as json")
}

async fn make_refresh_token_request(config: &Config, refresh_token: &str) -> Oauth2Token {
    let client = get_reqwest_client(config);

    let params = [
        ("refresh_token", refresh_token),
        ("grant_type", "refresh_token"),
    ];

    client
        .post("https://api.twitter.com/2/oauth2/token")
        .basic_auth(&config.client_id, Some(&config.client_secret))
        .header("content-type", "application/x-www-form-urlencoded")
        .form(&params)
        .send()
        .await
        .expect("Request failed")
        .json()
        .await
        .expect("Failed to get response as json")
}

async fn get_token_from_s3(client: &Client, config: &Config) -> GetObjectOutput {
    client
        .get_object()
        .bucket(&config.bucket_name)
        .key(S3_FILE_KEY)
        .send()
        .await
        .expect("Failed to get s3 object")
}

#[allow(dead_code)]
async fn store_token_in_s3(client: &Client, config: &Config, contents: &str) -> PutObjectOutput {
    client
        .put_object()
        .bucket(&config.bucket_name)
        .key(S3_FILE_KEY)
        .body(ByteStream::new(SdkBody::from(contents)))
        .send()
        .await
        .expect("Failed to put S3 object")
}
