use crate::client::Client;
use crate::default_client::{DefaultClient, DefaultClientError};
use crate::http_adapter::RestHttpAdapter;
use crate::socket::Socket;
use crate::web_socket::WebSocket;
use crate::web_socket_adapter::WebSocketAdapter;

pub enum NakamaError {
    ClientError(DefaultClientError<RestHttpAdapter>),
    // TODO
    // socket_error: <WebSocket<WebSocketAdapter> as Socket>::Error,
}

impl From<DefaultClientError<RestHttpAdapter>> for NakamaError {
    fn from(err: DefaultClientError<RestHttpAdapter>) -> Self {
        NakamaError::ClientError(err)
    }
}
