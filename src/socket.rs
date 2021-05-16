use crate::api::ApiChannelMessage;
use crate::session::Session;
use crate::socket_adapter::SocketAdapter;
use async_trait::async_trait;

pub struct WebSocketMessageEnvelope {
    pub cid: Option<String>,
}

#[async_trait(?Send)]
pub trait Socket {
    // It would make sense to have a future here
    fn on_closed<T>(&mut self, callback: T)
    where
        T: Fn() + 'static;

    fn on_connected<T>(&mut self, callback: T)
    where
        T: Fn() + 'static;

    fn on_received_channel_message<T>(&mut self, callback: T)
    where
        T: Fn(ApiChannelMessage) + 'static;

    async fn connect(&mut self, session: &mut Session, appear_online: bool, connect_timeout: i32);

    async fn close(&mut self);

    async fn create_match(&mut self);
}
