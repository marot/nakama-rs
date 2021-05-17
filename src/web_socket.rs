use crate::api::{ApiChannelMessage, ApiNotificationList, ApiRpc};
use crate::session::Session;
use crate::socket::{Socket, WebSocketMessageEnvelope, MatchCreate, Match, Channel, ChannelJoinMessage};
use crate::socket_adapter::SocketAdapter;
use async_trait::async_trait;
use std::error::Error;
use std::thread::sleep;
use std::time::Duration;
use nanoserde::{DeJson, SerJson};
use std::task::{Waker, Context, Poll};
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::future::Future;
use std::pin::Pin;

struct SharedState {
    cid: i64,
    wakers: HashMap<i64, Waker>,
    responses: HashMap<i64, WebSocketMessageEnvelope>,
}

struct WebSocketFuture {
    shared_state: Rc<RefCell<SharedState>>,
    cid: i64,
}

impl Future for WebSocketFuture {
    type Output = WebSocketMessageEnvelope;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut shared_state = self.shared_state.borrow_mut();
        if let Some(response) = shared_state.responses.remove(&self.cid)  {
            shared_state.wakers.remove(&self.cid);
            return Poll::Ready(response)
        }

        shared_state.wakers.insert(self.cid, cx.waker().clone());
        Poll::Pending
    }
}

struct WebSocket<A: SocketAdapter> {
    pub adapter: A,
    shared_state: Rc<RefCell<SharedState>>,
}

impl<A: SocketAdapter> WebSocket<A> {
    fn new(adapter: A) -> Self {
        let mut adapter = WebSocket { adapter,
        shared_state: Rc::new(RefCell::new(SharedState {
            cid: 1,
            wakers: HashMap::new(),
            responses: HashMap::new(),
        }))
        };

        adapter.adapter.on_received({
            let mut shared_state = adapter.shared_state.clone();
            move |msg| {
                println!("Msg: {:?}", msg);
                let msg = msg.unwrap();
                let event: WebSocketMessageEnvelope = DeJson::deserialize_json(&msg).unwrap();

                if let Some(ref cid) = event.cid {
                    let mut state = shared_state.borrow_mut();
                    let cid = cid.parse::<i64>().unwrap();
                    state.responses.insert(cid, event);
                    if let Some(waker) = state.wakers.remove(&cid) {
                        waker.wake();
                    }
                }
            }
        });

        adapter
    }

    fn tick(&self) {
        self.adapter.tick();
    }

    fn make_envelope_with_cid(&self) -> (WebSocketMessageEnvelope, i64) {
        self.shared_state.borrow_mut().cid += 1;
        let cid = self.shared_state.borrow().cid;

        (WebSocketMessageEnvelope {
            cid: Some(cid.to_string()),
            ..Default::default()
        }, cid)
    }

    async fn wait_response(&self, cid: i64) -> WebSocketMessageEnvelope {
        WebSocketFuture {
            shared_state: self.shared_state.clone(),
            cid,
        }.await
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

    async fn create_match(&self) -> Match {
        self.shared_state.borrow_mut().cid += 1;
        let cid = self.shared_state.borrow().cid;
        let envelope = WebSocketMessageEnvelope {
            cid: Some(cid.to_string()),
            match_create: Some(MatchCreate {}),
            ..Default::default()
        };

        let json = envelope.serialize_json();
        self.adapter.send(&json, false);

        let envelope = WebSocketFuture {
            shared_state: self.shared_state.clone(),
            cid,
        }.await;

        envelope.new_match.unwrap()
    }

    async fn join_chat(&self, room_name: &str, channel_type: i32, persistence: bool, hidden: bool) -> Channel {
        let (mut envelope, cid) = self.make_envelope_with_cid();
        envelope.channel_join = Some(ChannelJoinMessage {
            channel_type,
            hidden,
            persistence,
            target: room_name.to_owned(),
        });

        let json = envelope.serialize_json();
        self.adapter.send(&json, false);

        let result_envelope = self.wait_response(cid).await;
        result_envelope.channel.unwrap()
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
    use futures_timer::Delay;
    use futures::future::{select, Either};
    use futures::pin_mut;

    async fn tick<A: SocketAdapter>(web_socket: &WebSocket<A>) {
        web_socket.tick();
        Delay::new(Duration::from_millis(500)).await;
        web_socket.tick();
        Delay::new(Duration::from_millis(500)).await;
        web_socket.tick();
        Delay::new(Duration::from_millis(500)).await;
        web_socket.tick();
        Delay::new(Duration::from_millis(500)).await;
        web_socket.tick();
        Delay::new(Duration::from_millis(500)).await;
    }

    #[test]
    fn test() {
        // SimpleLogger::new().init().unwrap();
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

            sleep(Duration::from_secs(2));

            let a = web_socket.join_chat("MyRoom", 1, false, false);
            // let a = web_socket.create_match();
            let b = tick(&web_socket);

            pin_mut!(a);
            pin_mut!(b);

            match select(a, b).await {
                Either::Left((value1, _)) => {
                    println!("Channel: {:?}", value1);
                },
                Either::Right(_) => {
                }
            }
            sleep(Duration::from_secs(1));
        };

        futures::executor::block_on(future);
    }

    #[derive(SerJson)]
    struct TestStruct {
        a: Option<String>,
        b: Option<String>,
    }
    #[test]
    fn test_serialization() {
        let test_struct = TestStruct {
            a: Some("string".to_owned()),
            b: None,
        };
        let test_struct2 = TestStruct {
            a: None,
            b: Some("string".to_owned()),
        };
        let result = test_struct.serialize_json();
        let result2 = test_struct2.serialize_json();

        // This one is correct
        assert_eq!(result2, "{\"b\":\"string\"}");
        assert_eq!(result, "{\"a\":\"string\"}");
    }
}
