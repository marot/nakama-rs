use crate::web_socket::WebSocket;
use crate::web_socket_adapter::WebSocketAdapter;
use std::thread::{sleep, spawn};
use std::time::Duration;

pub fn tick_socket(socket: &WebSocket<WebSocketAdapter>) {
    spawn({
        let socket = socket.clone();
        move || loop {
            socket.tick();
            sleep(Duration::from_millis(16));
        }
    });
}
