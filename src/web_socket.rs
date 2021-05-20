use crate::api::ApiChannelMessage;
use crate::session::Session;
use crate::socket::{
    Channel, ChannelJoinMessage, ChannelSendMessage, Match, MatchCreate, Socket,
    WebSocketMessageEnvelope,
};
use crate::socket_adapter::SocketAdapter;
use async_trait::async_trait;
use log::{error, trace};
use nanoserde::{DeJson, DeJsonErr, SerJson};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};

use oneshot;

struct SharedState {
    cid: i64,
    wakers: HashMap<i64, Waker>,
    responses: HashMap<i64, WebSocketMessageEnvelope>,
    connected: Vec<oneshot::Sender<()>>,
}

struct WebSocketFuture {
    shared_state: Arc<Mutex<SharedState>>,
    cid: i64,
}

impl Future for WebSocketFuture {
    type Output = WebSocketMessageEnvelope;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // TODO: timeout
        let mut state = self.shared_state.lock().expect("Panic inside other mutex!");
        if let Some(response) = state.responses.remove(&self.cid) {
            state.wakers.remove(&self.cid);
            return Poll::Ready(response);
        }

        state.wakers.insert(self.cid, cx.waker().clone());
        Poll::Pending
    }
}

pub struct WebSocket<A: SocketAdapter> {
    adapter: Arc<Mutex<A>>,
    shared_state: Arc<Mutex<SharedState>>,
}

impl<A: SocketAdapter> Clone for WebSocket<A> {
    fn clone(&self) -> Self {
        WebSocket {
            adapter: self.adapter.clone(),
            shared_state: self.shared_state.clone(),
        }
    }
}

fn handle_message(shared_state: &Arc<Mutex<SharedState>>, msg: &String) {
    let result: Result<WebSocketMessageEnvelope, DeJsonErr> = DeJson::deserialize_json(&msg);
    match result {
        Ok(event) => {
            if let Some(ref cid) = event.cid {
                trace!("handle_message: Received message with cid");
                let mut state = shared_state.lock().expect("Panic inside other mutex!");
                let cid = cid.parse::<i64>().unwrap();
                state.responses.insert(cid, event);
                if let Some(waker) = state.wakers.remove(&cid) {
                    trace!("handle_message: Waking future");
                    waker.wake();
                }
            }
        }
        Err(err) => {
            error!("handle_message: Failed to parse json: {}", err);
        }
    }
}

impl<A: SocketAdapter> WebSocket<A> {
    pub fn new(adapter: A) -> Self {
        let web_socket = WebSocket {
            adapter: Arc::new(Mutex::new(adapter)),
            shared_state: Arc::new(Mutex::new(SharedState {
                cid: 1,
                wakers: HashMap::new(),
                responses: HashMap::new(),
                connected: vec![],
            })),
        };

        web_socket
            .adapter
            .lock()
            .expect("panic inside other mutex!")
            .on_received({
                let shared_state = web_socket.shared_state.clone();
                move |msg| match msg {
                    Err(error) => {
                        error!("on_received: {}", error);
                        return;
                    }
                    Ok(msg) => {
                        trace!("on_received: {}", msg);
                        handle_message(&shared_state, &msg);
                    }
                }
            });

        web_socket.adapter.lock().unwrap()
            .on_connected({
                let shared_state = web_socket.shared_state.clone();
                move || {
                    shared_state.lock().unwrap()
                        .connected.drain(..)
                        .for_each(|sender| { sender.send(()); });
                }
            });

        web_socket
    }

    pub fn tick(&self) {
        self.adapter
            .lock()
            .expect("panic inside other mutex!")
            .tick();
    }

    fn make_envelope_with_cid(&self) -> (WebSocketMessageEnvelope, i64) {
        let cid = {
            let mut state = self.shared_state.lock().expect("Panic inside other mutex!");
            state.cid += 1;
            state.cid
        };

        (
            WebSocketMessageEnvelope {
                cid: Some(cid.to_string()),
                ..Default::default()
            },
            cid,
        )
    }

    #[inline]
    fn send(&self, data: &str, reliable: bool) {
        self.adapter.lock().expect("panic inside other mutex!")
            .send(data, reliable);
    }

    async fn wait_response(&self, cid: i64) -> WebSocketMessageEnvelope {
        WebSocketFuture {
            shared_state: self.shared_state.clone(),
            cid,
        }
        .await
    }
}

#[async_trait]
impl<A: SocketAdapter + Send> Socket for WebSocket<A> {
    fn on_closed<T>(&mut self, _callback: T)
    where
        T: Fn() + 'static,
    {
        todo!()
    }

    fn on_connected<T>(&mut self, _callback: T)
    where
        T: Fn() + 'static,
    {
        todo!()
    }

    fn on_received_channel_message<T>(&mut self, _callback: T)
    where
        T: Fn(ApiChannelMessage) + 'static,
    {
        todo!()
    }

    async fn connect(&self, session: &mut Session, appear_online: bool, connect_timeout: i32) {
        let ws_url = "ws://127.0.0.1";
        let port = 7350;

        let ws_addr = format!(
            "{}:{}/ws?lang=en&status={}&token={}",
            ws_url, port, appear_online, session.auth_token,
        );

        let (tx, rx) = oneshot::channel();

        self.shared_state.lock().unwrap()
            .connected.push(tx);

        self.adapter
            .lock()
            .unwrap()
            .connect(&ws_addr, connect_timeout);

        rx.await;

        trace!("WebSocket::connect: connected!")
    }

    async fn close(&self) {
        todo!()
    }

    async fn create_match(&self) -> Match {
        let (mut envelope, cid) = self.make_envelope_with_cid();
        envelope.match_create = Some(MatchCreate {});

        let json = envelope.serialize_json();
        self.send(&json, false);

        let envelope = WebSocketFuture {
            shared_state: self.shared_state.clone(),
            cid,
        }
        .await;

        envelope.new_match.unwrap()
    }

    async fn write_chat_message(&self, channel_id: &str, content: &str) {
        let (mut envelope, cid) = self.make_envelope_with_cid();
        envelope.channel_message_send = Some(ChannelSendMessage {
            channel_id: channel_id.to_owned(),
            content: content.to_owned(),
        });

        let json = envelope.serialize_json();
        self.send(&json, false);

        // TODO: Message ack
        self.wait_response(cid).await;
        // let result_envelope = self.wait_response(cid).await;
        // result_envelope.channel.unwrap()
    }

    async fn join_chat(
        &self,
        room_name: &str,
        channel_type: i32,
        persistence: bool,
        hidden: bool,
    ) -> Channel {
        let (mut envelope, cid) = self.make_envelope_with_cid();
        envelope.channel_join = Some(ChannelJoinMessage {
            channel_type,
            hidden,
            persistence,
            target: room_name.to_owned(),
        });

        let json = envelope.serialize_json();
        self.send(&json, false);

        let result_envelope = self.wait_response(cid).await;
        result_envelope.channel.unwrap()
    }
}

#[cfg(test)]
mod test {
    use nanoserde::SerJson;
    #[derive(SerJson)]
    struct TestStruct {
        a: Option<String>,
        b: Option<String>,
        c: Option<String>,
    }
    #[test]
    fn test_serialization() {
        let test_struct = TestStruct {
            a: Some("string".to_owned()),
            b: Some("hello".to_owned()),
            c: None,
        };
        let test_struct2 = TestStruct {
            a: None,
            b: Some("string".to_owned()),
            c: Some("hello".to_owned()),
        };
        let result = test_struct.serialize_json();
        let result2 = test_struct2.serialize_json();

        // This one is correct
        assert_eq!(result2, "{\"b\":\"string\",\"c\":\"hello\"}");
        assert_eq!(result, "{\"a\":\"string\",\"b\":\"hello\"}");
    }
}
