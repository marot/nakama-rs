use crate::api;
use crate::api::{
    ApiAccount, ApiAccountApple, ApiAccountCustom, ApiAccountDevice, ApiAccountEmail,
    ApiAccountFacebook, ApiAccountGameCenter, ApiAccountGoogle, ApiSessionRefreshRequest,
    RestRequest,
};
use crate::api_gen::ApiSession;
use crate::client::Client;
use crate::http_adapter::{ClientAdapter, RestHttpAdapter};
use crate::session::Session;
use async_trait::async_trait;
use nanoserde::DeJson;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub struct DefaultClient<A: ClientAdapter> {
    adapter: A,
    server_key: String,
}

#[derive(DeJson)]
pub struct ClientError {
    error: String,
    code: i32,
    message: String,
}

impl DefaultClient<RestHttpAdapter> {
    pub fn new_with_adapter() -> DefaultClient<RestHttpAdapter> {
        let adapter = RestHttpAdapter::new("http://127.0.0.1", 7350);
        DefaultClient::new(adapter)
    }
}

impl<A: ClientAdapter + Send + Sync> DefaultClient<A> {
    pub fn new(adapter: A) -> DefaultClient<A> {
        DefaultClient {
            adapter,
            server_key: "defaultkey".to_owned(),
        }
    }

    #[inline]
    async fn send<T: DeJson + Send>(
        &self,
        request: RestRequest<T>,
    ) -> Result<T, DefaultClientError<A>> {
        self.adapter
            .send(request)
            .await
            .map_err(|err| DefaultClientError::HttpAdapterError(err))
    }

    fn map_session(api_session: ApiSession) -> Session {
        Session {
            auth_token: api_session.token,
            refresh_token: if api_session.refresh_token.len() == 0 {
                None
            } else {
                Some(api_session.refresh_token)
            },
        }
    }

    async fn _refresh_session(
        &self,
        session: &mut Session,
    ) -> Result<(), <DefaultClient<A> as Client>::Error> {
        // TODO: check expiration
        if let Some(refresh_token) = session.refresh_token.take() {
            let request = api::session_refresh(
                &self.server_key,
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
            DefaultClientError::HttpAdapterError(err) => std::fmt::Debug::fmt(err, f),
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

    /// Authenticate a user with an Apple ID against the server.
    ///
    /// Authenticate user with the ID `token` received from Apple.
    /// If the user does not exist and `create` is passed, the user is created with the optional `username`.
    /// `vars` can contain extra information that will be bundled in the session token.
    async fn authenticate_apple(
        &self,
        token: &str,
        username: Option<&str>,
        create: bool,
        vars: HashMap<String, String>,
    ) -> Result<Session, Self::Error> {
        let request = api::authenticate_apple(
            &self.server_key,
            "",
            ApiAccountApple {
                token: token.to_owned(),
                vars,
            },
            Some(create),
            username,
        );

        self.send(request)
            .await
            .map(DefaultClient::<A>::map_session)
    }

    /// Authenticate a user with a custom id.
    ///
    /// Authenticate user with a custom identifier usually obtained from an external authentication service.
    /// If the user does not exist and `create` is passed, the user is created with the optional `username`.
    /// `vars` can contain extra information that will be bundled in the session token.
    async fn authenticate_custom(
        &self,
        id: &str,
        username: Option<&str>,
        create: bool,
        vars: HashMap<String, String>,
    ) -> Result<Session, Self::Error> {
        let request = api::authenticate_custom(
            &self.server_key,
            "",
            ApiAccountCustom {
                id: id.to_owned(),
                vars,
            },
            Some(create),
            username,
        );

        self.send(request)
            .await
            .map(DefaultClient::<A>::map_session)
    }

    /// Authenticate a user with a device id.
    ///
    /// TODO: Mention minimum length requirements;
    /// Authenticate user with a device identifier usually obtained from a platform API.
    /// If the user does not exist and `create` is passed, the user is created with the optional `username`.
    /// `vars` can contain extra information that will be bundled in the session token.
    async fn authenticate_device(
        &self,
        id: &str,
        username: Option<&str>,
        create: bool,
        vars: HashMap<String, String>,
    ) -> Result<Session, Self::Error> {
        let request = api::authenticate_device(
            &self.server_key.clone(),
            "",
            ApiAccountDevice {
                id: id.to_owned(),
                vars,
            },
            Some(create),
            username,
        );

        self.send(request)
            .await
            .map(DefaultClient::<A>::map_session)
    }

    async fn authenticate_email(
        &self,
        email: &str,
        password: &str,
        username: Option<&str>,
        create: bool,
        vars: HashMap<String, String>,
    ) -> Result<Session, Self::Error> {
        let request = api::authenticate_email(
            &self.server_key.clone(),
            "",
            ApiAccountEmail {
                email: email.to_owned(),
                password: password.to_owned(),
                vars,
            },
            Some(create),
            username,
        );

        self.send(request)
            .await
            .map(DefaultClient::<A>::map_session)
    }

    async fn authenticate_facebook(
        &self,
        token: &str,
        username: Option<&str>,
        create: bool,
        vars: HashMap<String, String>,
        import: bool,
    ) -> Result<Session, Self::Error> {
        let request = api::authenticate_facebook(
            &self.server_key.clone(),
            "",
            ApiAccountFacebook {
                token: token.to_owned(),
                vars,
            },
            Some(create),
            username,
            Some(import),
        );

        self.send(request)
            .await
            .map(DefaultClient::<A>::map_session)
    }

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
    ) -> Result<Session, Self::Error> {
        let request = api::authenticate_game_center(
            &self.server_key.clone(),
            "",
            ApiAccountGameCenter {
                bundle_id: bundle_id.to_owned(),
                player_id: player_id.to_owned(),
                public_key_url: public_key_url.to_owned(),
                salt: salt.to_owned(),
                signature: signature.to_owned(),
                timestamp_seconds: timestamp.to_owned(),
                vars,
            },
            Some(create),
            username,
        );

        self.send(request)
            .await
            .map(DefaultClient::<A>::map_session)
    }

    async fn authenticate_google(
        &self,
        token: &str,
        username: Option<&str>,
        create: bool,
        vars: HashMap<String, String>,
    ) -> Result<Session, Self::Error> {
        let request = api::authenticate_google(
            &self.server_key.clone(),
            "",
            ApiAccountGoogle {
                token: token.to_owned(),
                vars,
            },
            Some(create),
            username,
        );

        self.send(request)
            .await
            .map(DefaultClient::<A>::map_session)
    }

    async fn authenticate_steam(
        &self,
        token: &str,
        username: Option<&str>,
        create: bool,
        vars: HashMap<String, String>,
    ) -> Result<Session, Self::Error> {
        let request = api::authenticate_google(
            &self.server_key.clone(),
            "",
            ApiAccountGoogle {
                token: token.to_owned(),
                vars,
            },
            Some(create),
            username,
        );

        self.send(request)
            .await
            .map(DefaultClient::<A>::map_session)
    }

    async fn add_friend_by_ids(
        &self,
        session: &mut Session,
        ids: &[&str],
    ) -> Result<(), Self::Error> {
        let ids: Vec<String> = ids.iter().map(|id| (*id).to_owned()).collect();
        let request = api::add_friends(&session.auth_token, &ids, &[]);

        self.send(request).await
    }

    async fn get_account(&self, session: &mut Session) -> Result<ApiAccount, Self::Error> {
        let request = api::get_account(&session.auth_token);
        self.send(request).await
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
