use crate::default_client::DefaultClientError;
use crate::http_adapter::RestHttpAdapter;

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
