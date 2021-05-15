use std::error::Error;
use std::net::ToSocketAddrs;

pub trait SocketAdapter<E: Error> {
    fn on_connected<T>(&mut self, callback: T)
    where
        T: Fn() + 'static;
    fn on_closed<T>(&mut self, callback: T)
    where
        T: Fn() + 'static;

    // TODO: correct error type
    fn on_received<T>(&mut self, callback: T)
    where
        T: Fn(Result<Vec<u8>, E>) + 'static;

    fn is_connected(&self) -> bool;
    fn is_connecting(&self) -> bool;

    fn close(&mut self);

    fn connect(&mut self, addr: &str, timeout: i32);

    fn send(&mut self, data: &[u8], reliable: bool);

    fn tick(&mut self);
}
