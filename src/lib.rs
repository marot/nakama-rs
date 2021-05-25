//! # Nakama rust client guide
//! ## Setup
//! ## Authenticate
//! 1. Build an instance of the client
//! ```
//! # use nakama_rs::client::Client;
//! # use nakama_rs::http_adapter::RestHttpAdapter;
//! # use nakama_rs::default_client::DefaultClient;
//! # use futures::executor::block_on;
//! # use std::collections::HashMap;
//! let adapter = RestHttpAdapter::new("http://127.0.0.1", 7350);
//! let client = DefaultClient::new(adapter);
//! ```
//! 2. Authenticate a user
//! ```
//! # use nakama_rs::client::Client;
//! # use nakama_rs::http_adapter::RestHttpAdapter;
//! # use nakama_rs::default_client::DefaultClient;
//! # use futures::executor::block_on;
//! # use std::collections::HashMap;
//! # let adapter = RestHttpAdapter::new("http://127.0.0.1", 7350);
//! # let client = DefaultClient::new(adapter);
//! block_on(async {
//!     client.authenticate_device("testdeviceid", None, true, HashMap::new()).await;
//! })
//! ```
//! ## Sessions
//! ## Send requests
//! ## Socket messages
//! ## Handle events
//! ## Logs and errors
//! ## Full example
mod api_gen;

pub mod config;

pub mod client;
pub mod default_client;
pub mod error;
pub mod helper;
pub mod http_adapter;
pub mod matchmaker;
pub mod session;
pub mod socket;
pub mod socket_adapter;
pub mod web_socket;
pub mod web_socket_adapter;

pub mod api {
    pub use super::api_gen::*;
}
