use nakama_macro::nakama_main;
use nakama_rs::client::Client;
use nakama_rs::default_client::{DefaultClient};
use nakama_rs::helper::tick_socket;
use nakama_rs::socket::{Socket, StatusPresenceEvent};
use nakama_rs::web_socket::WebSocket;
use std::collections::HashMap;
use std::sync::mpsc;

// This example demonstrates how to connect to a socket
#[nakama_main]
async fn main() {
    let client = DefaultClient::new_with_adapter();
    let mut socket = WebSocket::new_with_adapter();
    tick_socket(&socket);

    let (tx_presence, rx_presence) = mpsc::channel::<StatusPresenceEvent>();

    let mut session = client
        .authenticate_device("socket_example_id", None, true, HashMap::new())
        .await.expect("Failed to authenticate");

    socket.on_received_status_presence(move |presence| {
        tx_presence.send(presence).expect("Failed to send status presence");
    });

    socket.connect(&mut session, true, -1).await;

    let status_presence = rx_presence.recv().expect("Failed to receive status presence");
    println!("Status presence: {:?}", status_presence);
}
