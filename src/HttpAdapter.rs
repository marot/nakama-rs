use crate::api::RestRequest;
use quad_net::http_request::{HttpError, Method, Request, RequestBuilder};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::future::Future;
use std::net::ToSocketAddrs;

use crate::api;
use crate::api_gen;
use async_trait::async_trait;
use futures::TryFutureExt;
use nanoserde::{DeJson, DeJsonErr};

#[async_trait(?Send)]
trait HttpAdapter<T: DeJson, E: Error> {
    // TODO: Correct error type
    async fn send(&self, request: RestRequest<T>) -> Result<T, E>
    where
        T: 'async_trait;
}

#[derive(Debug)]
enum RestHttpError {
    HttpError(HttpError),
    JsonError(DeJsonErr),
}

impl Display for RestHttpError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl Error for RestHttpError {}

struct RestHttpAdapter {
    server: String,
    port: i32,
}

impl RestHttpAdapter {
    pub fn new(server: &str, port: i32) -> RestHttpAdapter {
        RestHttpAdapter {
            server: server.to_owned(),
            port,
        }
    }
}

#[async_trait(?Send)]
impl<T: DeJson> HttpAdapter<T, RestHttpError> for RestHttpAdapter {
    async fn send(&self, request: RestRequest<T>) -> Result<T, RestHttpError>
    where
        T: 'async_trait,
    {
        let auth_header = match request.authentication {
            api::Authentication::Basic { username, password } => {
                format!(
                    "Basic {}",
                    base64::encode(&format!("{}:{}", username, password))
                )
            }
            api::Authentication::Bearer { token } => {
                format!("Bearer {}", token)
            }
        };
        let method = match request.method {
            api::Method::Post => Method::Post,
            api::Method::Put => Method::Put,
            api::Method::Get => Method::Get,
            api::Method::Delete => Method::Delete,
        };

        let url = format!(
            "{}:{}{}?{}",
            self.server, self.port, request.urlpath, request.query_params
        );

        let request = RequestBuilder::new(&url)
            .method(method)
            .header("Authorization", &auth_header)
            .body(&request.body)
            .send()
            .map_err(|err| RestHttpError::HttpError(err));

        let response = request.await?;
        nanoserde::DeJson::deserialize_json(&response)
            .map_err(|json_err| RestHttpError::JsonError(json_err))
    }
}

trait SocketAdapter<E: Error> {
    fn on_connected<T>(&mut self, callback: T)
    where
        T: Fn();
    fn on_closed<T>(&mut self, callback: T)
    where
        T: Fn();

    // TODO: correct error type
    fn on_received<T>(&mut self, callback: T)
    where
        T: Fn(Result<Vec<u8>, E>);

    fn is_connected(&self) -> bool;
    fn is_connecting(&self) -> bool;

    fn close(&mut self);

    fn connect<A: ToSocketAddrs>(addr: A, timeout: i32);

    fn send(&mut self, data: &[u8], reliable: bool);
}

struct WebSocketAdapter {}

#[derive(Debug)]
enum WebSocketError {
    IOError,
    WSError,
}

impl Display for WebSocketError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl Error for WebSocketError {}

impl SocketAdapter<WebSocketError> for WebSocketAdapter {
    fn on_connected<T>(&mut self, callback: T)
    where
        T: Fn(),
    {
        todo!()
    }

    fn on_closed<T>(&mut self, callback: T)
    where
        T: Fn(),
    {
        todo!()
    }

    fn on_received<T>(&mut self, callback: T)
    where
        T: Fn(Result<Vec<u8>, WebSocketError>),
    {
        todo!()
    }

    fn is_connected(&self) -> bool {
        todo!()
    }

    fn is_connecting(&self) -> bool {
        todo!()
    }

    fn close(&mut self) {
        todo!()
    }

    fn connect<A: ToSocketAddrs>(addr: A, timeout: i32) {
        todo!()
    }

    fn send(&mut self, data: &[u8], reliable: bool) {
        todo!()
    }
}

struct Socket<A: SocketAdapter<E>, E: Error> {
    pub adapter: A,
    _marker: std::marker::PhantomData<E>,
}

impl<A: SocketAdapter<E>, E: Error> Socket<A, E> {
    fn new(adapter: A) -> Self {
        Socket {
            adapter,
            _marker: std::marker::PhantomData,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::api_gen::Authentication;
    use std::collections::HashMap;

    #[test]
    fn test() {
        let mut a = WebSocketAdapter {};

        let socket = Socket::new(a);

        let mut http = RestHttpAdapter::new("http://127.0.0.1", 7350);

        let request = api_gen::authenticate_device(
            "defaultkey",
            "",
            api::ApiAccountDevice {
                id: "SomeDeviceId".to_owned(),
                vars: HashMap::new(),
            },
            Some(true),
            Some("Marot"),
        );

        let future = http.send(request);

        let result = futures::executor::block_on(future);

        println!("Result {:?}", result);

        // a.on_connected(|| {
        //     println!("Hello World!");
        // })
    }
}
