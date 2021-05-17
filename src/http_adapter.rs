use crate::api::RestRequest;
use quad_net::http_request::{HttpError, Method, RequestBuilder};
use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::api;
use async_trait::async_trait;
use futures::TryFutureExt;
use nanoserde::{DeJson, DeJsonErr};

#[async_trait(?Send)]
pub trait HttpAdapter {
    type Error: Error;
    // TODO: Correct error type
    async fn send<T: DeJson>(&self, request: RestRequest<T>) -> Result<T, Self::Error>;
    // where
    //     T: 'async_trait;
}

#[derive(Debug)]
pub enum RestHttpError {
    HttpError(HttpError),
    JsonError(DeJsonErr),
}

impl Display for RestHttpError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl Error for RestHttpError {}

pub struct RestHttpAdapter {
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
impl HttpAdapter for RestHttpAdapter {
    type Error = RestHttpError;
    async fn send<T: DeJson>(&self, request: RestRequest<T>) -> Result<T, RestHttpError>
// where
    //     T: 'async_trait,
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
