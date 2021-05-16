use crate::socket_adapter::SocketAdapter;
use qws;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::net::ToSocketAddrs;
use std::pin::Pin;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc, Mutex};
use std::task::{Context, Poll};

pub struct WebSocketAdapter {
    on_connected: Option<Box<dyn Fn() + 'static>>,
    on_closed: Option<Box<dyn Fn() + 'static>>,
    on_received: Option<Box<dyn Fn(Result<String, WebSocketError>) + 'static>>,

    rx: Option<Receiver<String>>,
    sender: Option<qws::Sender>,
}

// Client on the websocket thread
struct WebSocketClient {
    tx: Sender<String>,
}

impl qws::Handler for WebSocketClient {
    fn on_message(&mut self, msg: qws::Message) -> qws::Result<()> {
        if let qws::Message::Text(data) = msg {
            self.tx.send(data);
        }
        Ok(())
    }
}

impl WebSocketAdapter {
    pub fn new() -> WebSocketAdapter {
        WebSocketAdapter {
            on_connected: None,
            on_closed: None,
            on_received: None,

            rx: None,
            sender: None,
        }
    }
}

#[derive(Debug)]
pub enum WebSocketError {
    IOError,
    WSError,
}

impl Display for WebSocketError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl Error for WebSocketError {}

impl SocketAdapter for WebSocketAdapter {
    type Error = WebSocketError;

    fn on_connected<T>(&mut self, callback: T)
    where
        T: Fn() + 'static,
    {
        self.on_connected = Some(Box::new(callback));
    }

    fn on_closed<T>(&mut self, callback: T)
    where
        T: Fn() + 'static,
    {
        self.on_closed = Some(Box::new(callback))
    }

    fn on_received<T>(&mut self, callback: T)
    where
        T: Fn(Result<String, WebSocketError>) + 'static,
    {
        self.on_received = Some(Box::new(callback));
    }

    fn is_connected(&self) -> bool {
        todo!()
    }

    fn is_connecting(&self) -> bool {
        todo!();
        false
    }

    fn close(&mut self) {
        todo!()
    }

    fn connect(&mut self, addr: &str, timeout: i32) {
        let (tx, rx) = mpsc::channel();
        let (tx_init, rx_init) = mpsc::channel();

        let addr = addr.to_owned();

        std::thread::spawn({
            move || {
                qws::connect(addr, |out| {
                    tx_init.send(out);
                    return WebSocketClient { tx: tx.clone() };
                })
            }
        });

        // Todo keep sender
        self.sender = rx_init.recv().ok();

        self.rx = Some(rx);
    }

    fn send(&self, data: &str, reliable: bool) {
        if let Some(ref sender) = self.sender {
            println!("Sending {:?}", data);
            let result = sender.send(qws::Message::Text(data.to_owned()));
            println!("Result {:?}", result);
        }
    }

    fn tick(&self) {
        if let Some(ref rx) = self.rx {
            while let Ok(data) = rx.try_recv() {
                if let Some(ref cb) = self.on_received {
                    cb(Ok(data));
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use simple_logger::SimpleLogger;
    use std::thread::sleep;
    use std::time::Duration;

    struct Foo {}

    impl Foo {
        pub fn new() -> Foo {
            Foo {}
        }

        pub fn on_connected(&self) {
            println!("on_connected");
        }
    }

    #[test]
    fn test() {
        SimpleLogger::new().init().unwrap();

        let foo = Foo::new();
        let mut socket_adapter = WebSocketAdapter::new();
        socket_adapter.connect("ws://echo.websocket.org", 0);
        socket_adapter.on_received(move |data| println!("{:?}", data));
        sleep(Duration::from_secs(1));

        println!("Sending!");
        socket_adapter.send("Hello", false);
        sleep(Duration::from_secs(1));
        println!("Tick!");
        socket_adapter.tick();
        sleep(Duration::from_secs(1));
        println!("Tick!");
        socket_adapter.tick();
    }
}
