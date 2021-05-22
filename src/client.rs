use crate::api::{ApiAccountDevice, ApiSessionRefreshRequest, RestRequest};
use crate::api_gen;
use crate::api_gen::ApiAccount;
use crate::http_adapter::ClientAdapter;
use crate::session::Session;
use async_trait::async_trait;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[async_trait]
pub trait Client {
    type Error: Error;

    async fn authenticate_apple(
        &self,
        token: &str,
        username: Option<&str>,
        create: bool,
        vars: HashMap<String, String>,
    ) -> Result<Session, Self::Error>;

    async fn authenticate_custom(
        &self,
        id: &str,
        username: Option<&str>,
        create: bool,
        vars: HashMap<String, String>,
    ) -> Result<Session, Self::Error>;

    async fn authenticate_device(
        &self,
        id: &str,
        username: Option<&str>,
        create: bool,
        vars: HashMap<String, String>,
    ) -> Result<Session, Self::Error>;

    async fn authenticate_email(
        &self,
        email: &str,
        password: &str,
        username: Option<&str>,
        create: bool,
        vars: HashMap<String, String>,
    ) -> Result<Session, Self::Error>;

    async fn authenticate_facebook(
        &self,
        token: &str,
        username: Option<&str>,
        create: bool,
        vars: HashMap<String, String>,
        import: bool,
    ) -> Result<Session, Self::Error>;

    async fn authenticate_game_center(
        &self,
        bundle_id: &str,
        player_id: &str,
        public_key_url: &str,
        salt: &str,
        signature: &str,
        timestamp: &str,
        username: Option<&str>,
        create: bool,
        vars: HashMap<String, String>,
    ) -> Result<Session, Self::Error>;

    async fn authenticate_google(
        &self,
        token: &str,
        username: Option<&str>,
        create: bool,
        vars: HashMap<String, String>,
    ) -> Result<Session, Self::Error>;

    async fn authenticate_steam(
        &self,
        token: &str,
        username: Option<&str>,
        create: bool,
        vars: HashMap<String, String>,
    ) -> Result<Session, Self::Error>;

    async fn add_friend_by_ids(
        &self,
        session: &mut Session,
        ids: &[&str],
    ) -> Result<(), Self::Error>;

    async fn get_account(&self, session: &mut Session) -> Result<ApiAccount, Self::Error>;
}
