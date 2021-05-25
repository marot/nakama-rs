use nakama_macro::nakama_main;
use nakama_rs::client::Client;
use nakama_rs::default_client::{DefaultClient, DefaultClientError};
use nakama_rs::error::NakamaError;
use nakama_rs::helper::tick_socket;
use nakama_rs::http_adapter::RestHttpAdapter;
use nakama_rs::http_adapter::RestHttpError::HttpError;
use nakama_rs::session::Session;
use nakama_rs::socket::Socket;
use nakama_rs::web_socket::WebSocket;
use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;

// This example demonstrates how to connect to a socket
#[nakama_main]
async fn main() -> Result<(), NakamaError> {
    let client = DefaultClient::new_with_adapter();
    let mut socket = WebSocket::new_with_adapter();
    tick_socket(&socket);

    let mut session = client
        .authenticate_device("socket_example_id", None, true, HashMap::new())
        .await?;

    socket.connect(&mut session, true, -1).await;

    let status_presence = socket.status_presence().await;
    println!("Status presence: {:?}", status_presence);
    Ok(())
}
