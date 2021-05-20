use crate::api::{ApiAccountDevice, ApiSessionRefreshRequest, RestRequest};
use crate::api_gen;
use crate::api_gen::ApiAccount;
use crate::client::DefaultClientError::HttpAdapterError;
use crate::http_adapter::ClientAdapter;
use crate::session::Session;
use async_trait::async_trait;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use nanoserde::DeJson;

#[async_trait]
pub trait Client {
    type Error: Error;
    async fn authenticate_device(
        &self,
        id: &str,
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

pub struct DefaultClient<A: ClientAdapter> {
    adapter: A,
}

impl<A: ClientAdapter> DefaultClient<A> {
    pub fn new(adapter: A) -> DefaultClient<A> {
        DefaultClient { adapter }
    }

    #[inline]
    async fn send<T: DeJson + Send>(&self, request: RestRequest<T>) -> Result<T, A::Error> {
       self.adapter.send(request).await
    }

    async fn _refresh_session(&self, session: &mut Session) -> Result<(), A::Error> {
        // TODO: check expiration
        if let Some(refresh_token) = session.refresh_token.take() {
            let request = api_gen::session_refresh(
                "defaultkey",
                "",
                ApiSessionRefreshRequest {
                    vars: HashMap::new(),
                    token: refresh_token,
                },
            );

            let sess = self.send(request).await;
            let result = sess.map(|s| {
                session.auth_token = s.token;
                session.refresh_token = if s.refresh_token.len() == 0 {
                    None
                } else {
                    Some(s.refresh_token)
                };
            });
            return result;
        }

        Ok(())
    }
}

pub enum DefaultClientError<A: ClientAdapter> {
    HttpAdapterError(A::Error),
    ClientError(String),
}

impl<A: ClientAdapter> Debug for DefaultClientError<A> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpAdapterError(err) => std::fmt::Debug::fmt(err, f),
            DefaultClientError::ClientError(err) => std::fmt::Debug::fmt(err, f),
        }
    }
}

impl<A: ClientAdapter> Display for DefaultClientError<A> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl<A: ClientAdapter> Error for DefaultClientError<A> {}

#[async_trait]
impl<A: ClientAdapter + Sync + Send> Client for DefaultClient<A> {
    type Error = DefaultClientError<A>;

    async fn authenticate_device(
        &self,
        id: &str,
        username: Option<&str>,
        create: bool,
        vars: HashMap<String, String>,
    ) -> Result<Session, Self::Error> {
        let request = api_gen::authenticate_device(
            "defaultkey",
            "",
            ApiAccountDevice {
                id: id.to_owned(),
                vars,
            },
            Some(create),
            username,
        );

        let response = self.send(request).await;
        response
            .map(|api_session| Session {
                auth_token: api_session.token,
                refresh_token: if api_session.refresh_token.len() == 0 {
                    None
                } else {
                    Some(api_session.refresh_token)
                },
            })
            .map_err(|err| HttpAdapterError(err))
    }

    async fn add_friend_by_ids(
        &self,
        session: &mut Session,
        ids: &[&str],
    ) -> Result<(), Self::Error> {
        let ids: Vec<String> = ids.iter().map(|id| (*id).to_owned()).collect();
        let request = api_gen::add_friends(&session.auth_token, &ids, &[]);

        self
            .send(request)
            .await
            .map_err(|err| DefaultClientError::HttpAdapterError(err))
    }

    async fn get_account(&self, session: &mut Session) -> Result<ApiAccount, Self::Error> {
        let request = api_gen::get_account(&session.auth_token);
        self
            .send(request)
            .await
            .map_err(|err| DefaultClientError::HttpAdapterError(err))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::http_adapter::RestHttpAdapter;

    #[test]
    fn test() {
        let adapter = RestHttpAdapter::new("http://127.0.0.1", 7350);
        let client = DefaultClient::new(adapter);

        let future = async {
            let mut session = client
                .authenticate_device("device_id_of_some_length", None, true, HashMap::new())
                .await?;
            let mut session_friend = client
                .authenticate_device("device_id2_of_some_length", None, true, HashMap::new())
                .await?;
            let account = client.get_account(&mut session_friend).await?;
            println!("Id: {:?}", account.user.id);
            client
                .add_friend_by_ids(&mut session, &[&account.user.id])
                .await
        };

        let result = futures::executor::block_on(future);

        println!("Result {:?}", result);
    }
}
