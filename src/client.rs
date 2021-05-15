use crate::api::{ApiAccountDevice, ApiSessionRefreshRequest};
use crate::api_gen;
use crate::api_gen::{ApiAccount, ApiSession};
use crate::http_adapter::HttpAdapter;
use crate::session::Session;
use async_trait::async_trait;
use std::collections::HashMap;
use std::error::Error;

#[async_trait(?Send)]
pub trait Client<E: Error> {
    async fn authenticate_device(
        &self,
        id: &str,
        username: Option<&str>,
        create: bool,
        vars: HashMap<String, String>,
    ) -> Result<Session, E>;

    async fn add_friend_by_ids(&self, session: &mut Session, ids: &[&str]) -> Result<(), E>;

    async fn get_account(&self, session: &mut Session) -> Result<ApiAccount, E>;
}

pub struct DefaultClient<A: HttpAdapter<E>, E: Error> {
    adapter: A,
    _marker: std::marker::PhantomData<E>,
}

impl<A: HttpAdapter<E>, E: Error> DefaultClient<A, E> {
    pub fn new(adapter: A) -> DefaultClient<A, E> {
        DefaultClient {
            adapter,
            _marker: std::marker::PhantomData,
        }
    }

    async fn refresh_session(&self, session: &mut Session) -> Result<(), E> {
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

            let sess = self.adapter.send(request).await;
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

#[async_trait(?Send)]
impl<A: HttpAdapter<E>, E: Error> Client<E> for DefaultClient<A, E> {
    async fn authenticate_device(
        &self,
        id: &str,
        username: Option<&str>,
        create: bool,
        vars: HashMap<String, String>,
    ) -> Result<Session, E> {
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

        let response = self.adapter.send(request).await;
        response.map(|api_session| Session {
            auth_token: api_session.token,
            refresh_token: if api_session.refresh_token.len() == 0 {
                None
            } else {
                Some(api_session.refresh_token)
            },
        })
    }

    async fn add_friend_by_ids(&self, session: &mut Session, ids: &[&str]) -> Result<(), E> {
        let ids: Vec<String> = ids.iter().map(|id| (*id).to_owned()).collect();
        let request = api_gen::add_friends(&session.auth_token, &ids, &[]);

        self.adapter.send(request).await
    }

    async fn get_account(&self, session: &mut Session) -> Result<ApiAccount, E> {
        let request = api_gen::get_account(&session.auth_token);
        self.adapter.send(request).await
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::http_adapter::RestHttpAdapter;

    #[test]
    fn test() {
        let adapter = RestHttpAdapter::new("http://127.0.0.1", 7350);
        let mut client = DefaultClient::new(adapter);

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
