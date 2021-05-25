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

async fn sockets_with_users(
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

#[test]
fn test_channel_room_creation() {
    let future = async {
        let (socket1, socket2, ..) = sockets_with_users("socketchannel1", "socketchannel2").await;
        let channel = socket1.join_chat("MyRoom", 1, false, false).await;
        assert_eq!(channel.unwrap().room_name, "MyRoom".to_owned())
    };

    block_on(future);
}

#[test]
fn test_channel_direct_message_creation() {
    SimpleLogger::new().init();
    let future = async {
        let (socket1, mut socket2, account1, account2) =
            sockets_with_users("socketchannel1", "socketchannel2").await;
        let channel = socket1.join_chat(&account2.user.id, 2, false, false).await;
        // The user will receive a notification that a user wants to chat and can then join.
        let _ = socket2.join_chat(&account1.user.id, 2, false, false).await;
        socket2.on_received_channel_presence(|presence| {
            println!("{:?}", presence);
        });
        // TODO: asyncify the callbacks for tests
        sleep(Duration::from_secs(1));
    };

    block_on(future);
}

#[test]
fn test_channel_group_chat_creation() {
    todo!()
}

#[test]
fn test_channel_leave() {
    block_on(async {
        let (socket1, socket2, ..) = sockets_with_users("socketchannel1", "socketchannel2").await;
        let channel = socket1.join_chat("MyRoom", 1, false, false).await.unwrap();
        socket1.leave_chat(&channel.id).await;
    });
}

#[test]
fn test_channel_message() {
    block_on(async {
        let (socket1, socket2, ..) = sockets_with_users("socketchannel1", "socketchannel2").await;
        let channel = socket1.join_chat("MyRoom", 1, true, false).await.unwrap();
        let ack = socket1
            .write_chat_message(&channel.id, r#"{"text":"Hello, World!"}"#)
            .await
            .unwrap();

        let ack = socket1
            .update_chat_message(&channel.id, &ack.message_id, r#"{"text":"Bye, World!"}"#)
            .await
            .unwrap();

        let ack = socket1
            .remove_chat_message(&channel.id, &ack.message_id)
            .await
            .unwrap();

        println!("{:?}", ack);
    })
}
