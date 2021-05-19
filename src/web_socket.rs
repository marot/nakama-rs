use crate::api::{ApiChannelMessage};
use crate::session::Session;
use crate::socket::{Socket, WebSocketMessageEnvelope, MatchCreate, Match, Channel, ChannelJoinMessage, ChannelSendMessage};
use crate::socket_adapter::SocketAdapter;
use async_trait::async_trait;
use nanoserde::{DeJson, SerJson, DeJsonErr};
use std::task::{Waker, Context, Poll};
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::future::Future;
use std::pin::Pin;
use log::{trace, error};
use std::sync::{Mutex, Arc};

struct SharedState {
    cid: i64,
    wakers: HashMap<i64, Waker>,
    responses: HashMap<i64, WebSocketMessageEnvelope>,
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
        if let Some(response) = state.responses.remove(&self.cid)  {
            state.wakers.remove(&self.cid);
            return Poll::Ready(response)
        }

        state.wakers.insert(self.cid, cx.waker().clone());
        Poll::Pending
    }
}

struct WebSocket<A: SocketAdapter> {
    pub adapter: Arc<Mutex<A>>,
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
    fn new(adapter: A) -> Self {
        let mut web_socket = WebSocket {
            adapter: Arc::new(Mutex::new(adapter)),
        shared_state: Arc::new(Mutex::new(SharedState {
            cid: 1,
            wakers: HashMap::new(),
            responses: HashMap::new(),
        }))
        };

        web_socket.adapter.lock().expect("panic inside other mutex!").
        on_received({
            let shared_state = web_socket.shared_state.clone();
            move |msg| {
                match msg {
                    Err(error) => {
                        error!("on_received: {}", error);
                        return;
                    },
                    Ok(msg) => {
                        trace!("on_received: {}", msg);
                        handle_message(&shared_state, &msg);
                    }
                }

            }
        });

        web_socket
    }

    fn tick(&self) {
        self.adapter.lock().expect("panic inside other mutex!").tick();
    }

    fn make_envelope_with_cid(&self) -> (WebSocketMessageEnvelope, i64) {
        let cid = {
            let mut state = self.shared_state.lock().expect("Panic inside other mutex!");
            state.cid += 1;
            state.cid
        };

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

    async fn connect(&mut self, session: &mut Session, appear_online: bool, connect_timeout: i32) {
        let ws_url = "ws://127.0.0.1";
        let port = 7350;

        let ws_addr = format!(
            "{}:{}/ws?lang=en&status={}&token={}",
            ws_url, port, appear_online, session.auth_token,
        );

        self.adapter.lock().unwrap().connect(&ws_addr, connect_timeout);
    }

    async fn close(&mut self) {
        todo!()
    }

    async fn create_match(&self) -> Match {
        let (mut envelope, cid) = self.make_envelope_with_cid();
        envelope.match_create = Some(MatchCreate {});

        let json = envelope.serialize_json();
        self.adapter.lock().unwrap().send(&json, false);

        let envelope = WebSocketFuture {
            shared_state: self.shared_state.clone(),
            cid,
        }.await;

        envelope.new_match.unwrap()
    }

    async fn write_chat_message(&self, channel_id: &str, content: &str) {
        let (mut envelope, cid) = self.make_envelope_with_cid();
        envelope.channel_message_send = Some(ChannelSendMessage {
            channel_id: channel_id.to_owned(),
            content: content.to_owned(),
        });

        let json = envelope.serialize_json();
        self.adapter.lock().unwrap().send(&json, false);

        // TODO: Message ack
        self.wait_response(cid).await;
        // let result_envelope = self.wait_response(cid).await;
        // result_envelope.channel.unwrap()
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
        self.adapter.lock().unwrap().send(&json, false);

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
    use std::time::Duration;
    use std::thread::{sleep, JoinHandle};
    use log::LevelFilter;
    use futures::executor::block_on;
    use std::sync::mpsc::{Sender, channel, RecvError};

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

    fn spawn_network_thread() -> (Sender<Pin<Box<dyn Future<Output=()> + Send>>>, Sender<()>) {
        let (tx, rx) = channel::<Pin<Box<dyn Future<Output=()> + Send>>>();
        let (tx_kill, rx_kill) = channel::<()>();
        std::thread::spawn({
            move || {
                loop {
                    let future = rx.try_recv();
                    match future {
                        Ok(future) => {
                            block_on(future);
                        }
                        Err(_) => {}
                    }

                    let kill = rx_kill.try_recv();
                    if kill.is_ok() {
                        return;
                    }

                    sleep(Duration::from_millis(100));
                }
            }
        });

        (tx, tx_kill)
    }

    #[test]
    fn web_socket_test() {
        SimpleLogger::new()
            .with_level(LevelFilter::Off)
            .with_module_level("nakama_rs", LevelFilter::Trace).init().unwrap();
        let http_adapter = RestHttpAdapter::new("http://127.0.0.1", 7350);
        let client = DefaultClient::new(http_adapter);
        let adapter = WebSocketAdapter::new();
        let adapter2 = WebSocketAdapter::new();
        let mut web_socket = WebSocket::new(adapter);
        let mut web_socket2 = WebSocket::new(adapter2);

        let (send_futures, send_kill) = spawn_network_thread();

        let mut sockets = (web_socket, web_socket2);

        let setup = {
            let mut sockets = sockets.clone();
            async move {
                let session = client
                    .authenticate_device("testdeviceid", None, true, HashMap::new())
                    .await;
                let session2 = client.authenticate_device("testdeviceid2", None, true, HashMap::new())
                    .await;
                let mut session = session.unwrap();
                let mut session2 = session2.unwrap();
                sockets.0.connect(&mut session, true, -1).await;
                sockets.1.connect(&mut session2, true, -1).await;

                // Wait for connection
                sleep(Duration::from_secs(2));
            }
        };

        block_on(setup);

        let do_some_chatting = Box::pin({
            let sockets = sockets.clone();
            async move {
                sockets.0.join_chat("MyRoom", 1, false, false).await;
                let channel = sockets.1.join_chat("MyRoom", 1, false, false).await;
                sockets.1.write_chat_message(&channel.id, "{\"text\":\"Hello World!\"}").await;
            }
        });

        send_futures.send(do_some_chatting);

        // let join_handle = std::thread::spawn({
        //     move || {
        //         futures::executor::block_on(do_some_chatting);
        //     }
        // });

        let mut total_time = 0;
        while total_time < 5 * 1000 {
            sockets.0.tick();
            sockets.1.tick();
            sleep(Duration::from_millis(60));
            total_time += 60;
        }


        send_kill.send(());
        // join_handle.join();
    }

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
