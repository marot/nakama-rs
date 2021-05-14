use nakama_rs::{api_client::ApiClient, config};

use futures::executor::block_on;

fn main() {
    let mut client = ApiClient::new("defaultkey", "127.0.0.1", config::DEFAULT_PORT, "http");

    let future = client.authenticate_async("email@provider.com", "password");

    let result = block_on(future);
    println!("{:?}", result);
}
