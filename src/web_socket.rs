use crate::api::ApiChannelMessage;
use crate::session::Session;
use crate::socket::Socket;
use crate::socket_adapter::SocketAdapter;
use async_trait::async_trait;
use std::error::Error;
use std::thread::sleep;
use std::time::Duration;

struct WebSocket<A: SocketAdapter> {
    pub adapter: A,
}

impl<A: SocketAdapter> WebSocket<A> {
    fn new(adapter: A) -> Self {
        WebSocket { adapter }
    }
}

#[async_trait(?Send)]
impl<A: SocketAdapter> Socket for WebSocket<A> {
    fn on_closed<T>(&mut self, callback: T)
    where
        T: Fn() + 'static,
    {
        todo!()
    }

    fn on_connected<T>(&mut self, callback: T)
    where
        T: Fn() + 'static,
    {
        todo!()
    }

    fn on_received_channel_message<T>(&mut self, callback: T)
    where
        T: Fn(ApiChannelMessage) + 'static,
    {
        todo!()
    }

    async fn connect(&mut self, session: &mut Session, appear_online: bool, connect_timeout: i32) {
        let ws_url = "ws://127.0.0.1";
        let port = 7350;

        let ws_addr = format!(
            "{}:{}/ws?lang=en&status={}&token={}",
            ws_url, port, appear_online, session.auth_token,
        );

        self.adapter.connect(&ws_addr, connect_timeout);
    }

    async fn close(&mut self) {
        todo!()
    }

    async fn create_match(&mut self) {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::client::{Client, DefaultClient};
    use crate::http_adapter::RestHttpAdapter;
    use crate::web_socket_adapter::WebSocketAdapter;
    use simple_logger::SimpleLogger;
    use std::collections::HashMap;

    #[test]
    fn test() {
        SimpleLogger::new().init().unwrap();
        let http_adapter = RestHttpAdapter::new("http://127.0.0.1", 7350);
        let client = DefaultClient::new(http_adapter);
        let adapter = WebSocketAdapter::new();
        let mut web_socket = WebSocket::new(adapter);

        let future = async {
            let mut session = client
                .authenticate_device("testdeviceid", None, true, HashMap::new())
                .await;
            if let Err(ref err) = session {
                println!("Error: {:?}", err);
                return;
            }
            let mut session = session.unwrap();
            web_socket.connect(&mut session, true, -1).await;
            sleep(Duration::from_secs(1));
        };

        futures::executor::block_on(future);
    }
}
