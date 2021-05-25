use futures::executor::block_on;
use nakama_rs::client::Client;
use nakama_rs::default_client::DefaultClient;
use nakama_rs::error::NakamaError;
use nakama_rs::helper::tick_socket;
use nakama_rs::socket::{ChannelJoinType, Socket};
use nakama_rs::web_socket::WebSocket;

use nakama_rs::api::ApiAccount;
use nakama_rs::session::Session;
use nakama_rs::web_socket_adapter::WebSocketAdapter;
use simple_logger::SimpleLogger;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::mpsc;
use std::thread::sleep;
use std::time::Duration;

pub async fn sockets_with_users(
    id_one: &str,
    id_two: &str,
) -> (
    WebSocket<WebSocketAdapter>,
    WebSocket<WebSocketAdapter>,
    ApiAccount,
    ApiAccount,
) {
    let client = DefaultClient::new_with_adapter();
    let socket = WebSocket::new_with_adapter();
    let socket2 = WebSocket::new_with_adapter();
    tick_socket(&socket);
    tick_socket(&socket2);

    let mut session = client
        .authenticate_device(id_one, Some(id_one.clone()), true, HashMap::new())
        .await
        .unwrap();
    let mut session2 = client
        .authenticate_device(id_two, Some(id_two.clone()), true, HashMap::new())
        .await
        .unwrap();

    let account1 = client.get_account(&mut session).await.unwrap();
    let account2 = client.get_account(&mut session2).await.unwrap();

    socket.connect(&mut session, true, -1).await;
    socket2.connect(&mut session2, true, -1).await;

    (socket, socket2, account1, account2)
}