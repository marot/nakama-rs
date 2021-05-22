use futures::executor::block_on;
use nakama_rs::client::Client;
use nakama_rs::default_client::{DefaultClient, DefaultClientError};
use nakama_rs::http_adapter::RestHttpAdapter;
use nakama_rs::http_adapter::RestHttpError::HttpError;
use nakama_rs::session::Session;
use simple_logger::SimpleLogger;
use std::collections::HashMap;
use std::io::Error;

fn main() {
    let client = DefaultClient::new_with_adapter();

    let result = block_on(async {
        client
            .authenticate_device("too_short", None, true, HashMap::new())
            .await
    });

    println!("{:?}", result);

    let result = block_on(async {
        client
            .authenticate_device("long_enough_device_id", None, true, HashMap::new())
            .await
    });

    println!("{:?}", result);
}
