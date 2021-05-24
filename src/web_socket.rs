use crate::api::{ApiChannelMessage, ApiNotification, ApiRpc};
use crate::session::Session;
use crate::socket::{Channel, ChannelJoin, Match, MatchCreate, Socket, WebSocketMessageEnvelope, ChannelMessageSend, PartyLeader, PartyMatchmakerTicket, PartyClose, PartyData, MatchmakerMatched, Error, UserPresence, MatchData, MatchPresenceEvent, StatusPresenceEvent, PartyPresenceEvent, ChannelPresenceEvent, StreamPresenceEvent, StreamData, PartyJoinRequest, PartyAccept, Status, Party, ChannelMessageAck, MatchmakerTicket, MatchmakerAdd, PartyMatchmakerAdd, PartyCreate, StatusFollow, StatusUpdate, ChannelMesageUpdate, StatusUnfollow, PartyDataSend, MatchDataSend, PartyRemove, PartyMatchmakerRemove, MatchmakerRemove, ChannelMesageRemove, PartyPromote, PartyJoinRequestList, PartyLeave, MatchLeave, ChannelLeave, MatchJoin, PartyJoin};
use crate::socket_adapter::SocketAdapter;
use async_trait::async_trait;
use log::{error, trace};
use nanoserde::{DeJson, DeJsonErr, SerJson};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};

use oneshot;
use crate::default_client::str_slice_to_owned;

struct SharedState {
    cid: i64,
    wakers: HashMap<i64, Waker>,
    connected: Vec<oneshot::Sender<()>>,
    responses: HashMap<i64, oneshot::Sender<WebSocketMessageEnvelope>>,
}

pub struct WebSocket<A: SocketAdapter> {
    adapter: Arc<Mutex<A>>,
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
                if let Some(response_event) = state.responses.remove(&cid) {
                    response_event.send(event);
                }
            }
        }
        Err(err) => {
            error!("handle_message: Failed to parse json: {}", err);
        }
    }
}

impl<A: SocketAdapter> WebSocket<A> {
    pub fn new(adapter: A) -> Self {
        let web_socket = WebSocket {
            adapter: Arc::new(Mutex::new(adapter)),
            shared_state: Arc::new(Mutex::new(SharedState {
                cid: 1,
                wakers: HashMap::new(),
                responses: HashMap::new(),
                connected: vec![],
            })),
        };

        web_socket
            .adapter
            .lock()
            .expect("panic inside other mutex!")
            .on_received({
                let shared_state = web_socket.shared_state.clone();
                move |msg| match msg {
                    Err(error) => {
                        error!("on_received: {}", error);
                        return;
                    }
                    Ok(msg) => {
                        trace!("on_received: {}", msg);
                        handle_message(&shared_state, &msg);
                    }
                }
            });

        web_socket.adapter.lock().unwrap()
            .on_connected({
                let shared_state = web_socket.shared_state.clone();
                move || {
                    shared_state.lock().unwrap()
                        .connected.drain(..)
                        .for_each(|sender| { sender.send(()); });
                }
            });

        web_socket
    }

    pub fn tick(&self) {
        self.adapter
            .lock()
            .expect("panic inside other mutex!")
            .tick();
    }

    fn make_envelope_with_cid(&self) -> (WebSocketMessageEnvelope, i64) {
        let cid = {
            let mut state = self.shared_state.lock().expect("Panic inside other mutex!");
            state.cid += 1;
            state.cid
        };

        (
            WebSocketMessageEnvelope {
                cid: Some(cid.to_string()),
                ..Default::default()
            },
            cid,
        )
    }

    #[inline]
    fn send(&self, data: &str, reliable: bool) {
        self.adapter.lock().expect("panic inside other mutex!")
            .send(data, reliable);
    }

    async fn wait_response(&self, cid: i64) -> WebSocketMessageEnvelope {
        let (tx, rx) = oneshot::channel::<WebSocketMessageEnvelope>();

        {
            let mut shared_state = self.shared_state.lock().unwrap();
            shared_state.responses.insert(cid, tx);
        }

        rx.await.unwrap()
    }
}

#[async_trait]
impl<A: SocketAdapter + Send> Socket for WebSocket<A> {
    fn on_closed<T>(&mut self, callback: T)
    where
        T: Fn() + Send + 'static,
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

    fn on_received_channel_presence<T>(&mut self, callback: T) where
        T: Fn(ChannelPresenceEvent) + 'static {
        todo!()
    }

    fn on_received_error<T>(&mut self, callback: T) where
        T: Fn(Error) + 'static {
        todo!()
    }

    fn on_received_matchmaker_matched<T>(&mut self, callback: T) where
        T: Fn(MatchmakerMatched) + 'static {
        todo!()
    }

    fn on_received_match_state<T>(&mut self, callback: T) where
        T: Fn(MatchData) + 'static {
        todo!()
    }

    fn on_received_match_presence<T>(&mut self, callback: T) where
        T: Fn(MatchPresenceEvent) + 'static {
        todo!()
    }

    fn on_received_notification<T>(&mut self, callback: T) where
        T: Fn(ApiNotification) + 'static {
        todo!()
    }

    fn on_received_party_close<T>(&mut self, callback: T) where
        T: Fn(PartyClose) + 'static {
        todo!()
    }

    fn on_received_party_data<T>(&mut self, callback: T) where
        T: Fn(PartyData) + 'static {
        todo!()
    }

    fn on_received_party_join_request<T>(&mut self, callback: T) where
        T: Fn(PartyJoinRequest) + 'static {
        todo!()
    }

    fn on_received_party_leader<T>(&mut self, callback: T) where
        T: Fn(PartyLeader) + 'static {
        todo!()
    }

    fn on_received_party_presence<T>(&mut self, callback: T) where
        T: Fn(PartyPresenceEvent) + 'static {
        todo!()
    }

    fn on_received_status_presence<T>(&mut self, callback: T) where
        T: Fn(StatusPresenceEvent) + 'static {
        todo!()
    }

    fn on_received_stream_presence<T>(&mut self, callback: T) where
        T: Fn(StreamPresenceEvent) + 'static {
        todo!()
    }

    fn on_received_stream_state<T>(&mut self, callback: T) where
        T: Fn(StreamData) + 'static {
        todo!()
    }

    async fn accept_party_member(&self, party_id: &str, user_presence: &UserPresence) {
        let (mut envelope, cid) = self.make_envelope_with_cid();
        envelope.party_accept = Some(PartyAccept {
            party_id: party_id.to_owned(),
            presence: user_presence.clone(),
        });

        let json = envelope.serialize_json();
        self.send(&json, false);
    }

    async fn add_matchmaker(&self, query: &str, min_count: Option<i32>, max_count: Option<i32>, string_properties: HashMap<String, String>, numeric_properties: HashMap<String, f64>) -> MatchmakerTicket {
        let (mut envelope, cid) = self.make_envelope_with_cid();
        envelope.matchmaker_add = Some(MatchmakerAdd {
            query: query.to_owned(),
            min_count: min_count.unwrap_or(2),
            max_count: max_count.unwrap_or(8),
            numeric_properties,
            string_properties,
        });

        let json = envelope.serialize_json();
        self.send(&json, false);

        let envelope = self.wait_response(cid).await;

        envelope.matchmaker_ticket.unwrap()
    }

    async fn add_matchmaker_party(&self, party_id: &str, query: &str, min_count: i32, max_count: i32, string_properties: HashMap<String, String>, numeric_properties: HashMap<String, f64>) -> PartyMatchmakerTicket{
        let (mut envelope, cid) = self.make_envelope_with_cid();
        envelope.party_matchmaker_add = Some(PartyMatchmakerAdd {
            query: query.to_owned(),
            min_count: min_count,
            max_count: max_count,
            numeric_properties,
            string_properties,
            party_id: party_id.to_owned(),
        });

        let json = envelope.serialize_json();
        self.send(&json, false);

        let envelope = self.wait_response(cid).await;

        envelope.party_matchmaker_ticket.unwrap()
    }

    async fn close_party(&self, party_id: &str) {
        let (mut envelope, cid) = self.make_envelope_with_cid();
        envelope.party_close = Some(PartyClose {
            party_id: party_id.to_owned()
        });

        let json = envelope.serialize_json();
        self.send(&json, false);
    }

    async fn close(&self) {
        todo!()
    }

    async fn connect(&self, session: &mut Session, appear_online: bool, connect_timeout: i32) {
        let ws_url = "ws://127.0.0.1";
        let port = 7350;

        let ws_addr = format!(
            "{}:{}/ws?lang=en&status={}&token={}",
            ws_url, port, appear_online, session.auth_token,
        );

        let (tx, rx) = oneshot::channel();

        self.shared_state.lock().unwrap()
            .connected.push(tx);

        self.adapter
            .lock()
            .unwrap()
            .connect(&ws_addr, connect_timeout);

        rx.await;
    }

    async fn create_match(&self) -> Match {
        let (mut envelope, cid) = self.make_envelope_with_cid();
        envelope.match_create = Some(MatchCreate {});

        let json = envelope.serialize_json();
        self.send(&json, false);

        let envelope = self.wait_response(cid).await;

        envelope.new_match.unwrap()
    }

    async fn create_party(&self, open: bool, max_size: i32) -> Party {
        let (mut envelope, cid) = self.make_envelope_with_cid();
        envelope.party_create = Some(PartyCreate {
            max_size,
            open,
        });

        let json = envelope.serialize_json();
        self.send(&json, false);

        let result_envelope = self.wait_response(cid).await;
        result_envelope.party.unwrap()
    }

    async fn follow_users(&self, user_ids: &[&str], usernames: &[&str]) -> Status {
        let (mut envelope, cid) = self.make_envelope_with_cid();
        envelope.status_follow = Some(StatusFollow {
            user_ids: str_slice_to_owned(user_ids),
            usernames: str_slice_to_owned(usernames),
        });

        let json = envelope.serialize_json();
        self.send(&json, false);

        let result_envelope = self.wait_response(cid).await;
        result_envelope.status.unwrap()
    }

    async fn join_chat(
        &self,
        room_name: &str,
        channel_type: i32,
        persistence: bool,
        hidden: bool,
    ) -> Channel {
        let (mut envelope, cid) = self.make_envelope_with_cid();
        envelope.channel_join = Some(ChannelJoin {
            channel_type,
            hidden,
            persistence,
            target: room_name.to_owned(),
        });

        let json = envelope.serialize_json();
        self.send(&json, false);

        let result_envelope = self.wait_response(cid).await;
        result_envelope.channel.unwrap()
    }

    async fn join_party(&self, party_id: &str) {
        let (mut envelope, cid) = self.make_envelope_with_cid();
        envelope.party_join = Some(PartyJoin {
            party_id: party_id.to_owned(),
        });

        let json = envelope.serialize_json();
        self.send(&json, false);
    }

    async fn join_match(&self, matched: MatchmakerMatched) -> Match {
        let (mut envelope, cid) = self.make_envelope_with_cid();
        envelope.match_join = Some(MatchJoin {
            token: matched.token,
            match_id: matched.match_id,
            metadata: HashMap::new(),
        });

        let json = envelope.serialize_json();
        self.send(&json, false);

        let result_envelope = self.wait_response(cid).await;
        result_envelope.new_match.unwrap()
    }

    async fn join_match_by_id(&self, match_id: &str, metadata: HashMap<String, String>) -> Match {
        let (mut envelope, cid) = self.make_envelope_with_cid();
        envelope.match_join = Some(MatchJoin {
            match_id: Some(match_id.to_owned()),
            token: None,
            metadata,
        });

        let json = envelope.serialize_json();
        self.send(&json, false);

        let result_envelope = self.wait_response(cid).await;
        result_envelope.new_match.unwrap()
    }

    async fn leave_chat(&self, channel_id: &str) {
        let (mut envelope, cid) = self.make_envelope_with_cid();
        envelope.channel_leave = Some(ChannelLeave {
            channel_id: channel_id.to_owned(),
        });

        let json = envelope.serialize_json();
        self.send(&json, false);
    }

    async fn leave_match(&self, match_id: &str) {
        let (mut envelope, cid) = self.make_envelope_with_cid();
        envelope.match_leave = Some(MatchLeave {
            match_id: match_id.to_owned(),
        });

        let json = envelope.serialize_json();
        self.send(&json, false);
    }

    async fn leave_party(&self, party_id: &str) {
        let (mut envelope, cid) = self.make_envelope_with_cid();
        envelope.party_leave = Some(PartyLeave {
            party_id: party_id.to_owned(),
        });

        let json = envelope.serialize_json();
        self.send(&json, false);
    }

    async fn list_party_join_requests(&self, party_id: &str) -> PartyJoinRequest {
        let (mut envelope, cid) = self.make_envelope_with_cid();
        envelope.party_join_request_list = Some(PartyJoinRequestList {
            party_id: party_id.to_owned()
        });

        let json = envelope.serialize_json();
        self.send(&json, false);

        let result_envelope = self.wait_response(cid).await;
        result_envelope.party_join_request.unwrap()
    }

    async fn promote_party_member(&self, party_id: &str, party_member: UserPresence) {
        let (mut envelope, cid) = self.make_envelope_with_cid();
        envelope.party_promote = Some(PartyPromote {
            party_id: party_id.to_owned(),
            presence: party_member,
        });

        let json = envelope.serialize_json();
        self.send(&json, false);
    }

    async fn remove_chat_message(&self, channel_id: &str, message_id: &str) -> ChannelMessageAck {
        let (mut envelope, cid) = self.make_envelope_with_cid();
        envelope.channel_message_remove = Some(ChannelMesageRemove {
            channel_id: channel_id.to_owned(),
            message_id: message_id.to_owned(),
        });

        let json = envelope.serialize_json();
        self.send(&json, false);

        let result_envelope = self.wait_response(cid).await;
        result_envelope.channel_message_ack.unwrap()
    }

    async fn remove_matchmaker(&self, ticket: &str) {
        let (mut envelope, cid) = self.make_envelope_with_cid();
        envelope.matchmaker_remove = Some(MatchmakerRemove {
            ticket: ticket.to_owned(),
        });

        let json = envelope.serialize_json();
        self.send(&json, false);
    }

    async fn remove_matchmaker_party(&self, party_id: &str, ticket: &str) {
        let (mut envelope, cid) = self.make_envelope_with_cid();
        envelope.party_matchmaker_remove = Some(PartyMatchmakerRemove {
            party_id: party_id.to_owned(),
            ticket: ticket.to_owned(),
        });

        let json = envelope.serialize_json();
        self.send(&json, false);
    }

    async fn remove_party_member(&self, party_id: &str, presence: UserPresence) {
        let (mut envelope, cid) = self.make_envelope_with_cid();
        envelope.party_remove = Some(PartyRemove {
            party_id: party_id.to_owned(),
            presence,
        });

        let json = envelope.serialize_json();
        self.send(&json, false);
    }

    async fn rpc(&self, func_id: &str, payload: &str) -> ApiRpc {
        let (mut envelope, cid) = self.make_envelope_with_cid();
        envelope.rpc = Some(ApiRpc {
            id: func_id.to_owned(),
            http_key: "".to_owned(),
            payload: payload.to_owned(),
        });

        let json = envelope.serialize_json();
        self.send(&json, false);

        let result_envelope = self.wait_response(cid).await;
        result_envelope.rpc.unwrap()
    }

    async fn rpc_bytes(&self, func_id: &str, payload: &[u8]) -> ApiRpc {
        let (mut envelope, cid) = self.make_envelope_with_cid();
        envelope.rpc = Some(ApiRpc {
            id: func_id.to_owned(),
            http_key: "".to_owned(),
            // TODO: How to convert to string
            payload: "".to_owned(),
        });

        let json = envelope.serialize_json();
        self.send(&json, false);

        let result_envelope = self.wait_response(cid).await;
        result_envelope.rpc.unwrap()
    }

    async fn send_match_state(&self, match_id: &str, op_code: i64, state: &[u8], presences: &[UserPresence]) {
        let (mut envelope, cid) = self.make_envelope_with_cid();
        envelope.match_data_send = Some(MatchDataSend {
            match_id: match_id.to_owned(),
            op_code,
            data: state.to_vec(),
            presences: presences.to_vec(),
            // TODO: Reliable?
            reliable: false,
        });

        let json = envelope.serialize_json();
        self.send(&json, false);
    }

    async fn send_party_data(&self, party_id: &str, op_code: i64, data: &[u8]) {
        let (mut envelope, cid) = self.make_envelope_with_cid();
        envelope.party_data_send = Some(PartyDataSend {
            party_id: party_id.to_owned(),
            op_code,
            data: data.to_vec(),
        });

        let json = envelope.serialize_json();
        self.send(&json, false);
    }

    async fn unfollow_users(&self, user_ids: &[&str]) {
        let (mut envelope, cid) = self.make_envelope_with_cid();
        envelope.status_unfollow = Some(StatusUnfollow {
            user_ids: str_slice_to_owned(user_ids),
        });

        let json = envelope.serialize_json();
        self.send(&json, false);
    }

    async fn update_chat_message(&self, channel_id: &str, message_id: &str, content: &str) -> ChannelMessageAck {
        let (mut envelope, cid) = self.make_envelope_with_cid();
        envelope.channel_message_update = Some(ChannelMesageUpdate {
            channel_id: channel_id.to_owned(),
            message_id: message_id.to_owned(),
            content: content.to_owned(),
        });

        let json = envelope.serialize_json();
        self.send(&json, false);

        let result_envelope = self.wait_response(cid).await;
        result_envelope.channel_message_ack.unwrap()
    }

    async fn update_status(&self, status: &str) {
        let (mut envelope, cid) = self.make_envelope_with_cid();
        envelope.status_update = Some(StatusUpdate {
            status: status.to_owned(),
        });

        let json = envelope.serialize_json();
        self.send(&json, false);
    }

    async fn write_chat_message(&self, channel_id: &str, content: &str) {
        let (mut envelope, cid) = self.make_envelope_with_cid();
        envelope.channel_message_send = Some(ChannelMessageSend {
            channel_id: channel_id.to_owned(),
            content: content.to_owned(),
        });

        let json = envelope.serialize_json();
        self.send(&json, false);

        // TODO: Message ack
        self.wait_response(cid).await;
    }
}

#[cfg(test)]
mod test {
    use nanoserde::SerJson;
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
