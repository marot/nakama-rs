use nakama_macro::nakama_main;
use nakama_rs::client::Client;
use nakama_rs::default_client::{DefaultClient, DefaultClientError};
use nakama_rs::http_adapter::RestHttpAdapter;
use nakama_rs::http_adapter::RestHttpError::HttpError;
use nakama_rs::session::Session;
use std::collections::HashMap;
use std::io::Error;

#[nakama_main]
async fn main() {
    let client = DefaultClient::new_with_adapter();

    let result = client
        .authenticate_device("too_short", None, true, HashMap::new())
        .await;

    let result = client
        .authenticate_device("long_enough_device_id", None, true, HashMap::new())
        .await;

    println!("{:?}", result);
}
