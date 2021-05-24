use crate::api::{ApiChannelMessage, ApiNotification, ApiRpc, ApiNotificationList};
use crate::session::Session;
use async_trait::async_trait;
use nanoserde::{DeJson, SerJson, DeJsonErr};
use std::collections::HashMap;

#[derive(DeJson, SerJson, Debug, Clone, Default)]
pub struct Timestamp(String);


#[derive(DeJson, SerJson, Debug, Clone, Default)]
pub struct Channel {
    pub id: String,
    pub presences: Vec<UserPresence>,
    #[nserde(rename = "self")]
    pub _self: UserPresence,
    pub room_name: Option<String>,
    pub group_id: Option<String>,
    pub user_id_one: Option<String>,
    pub user_id_two: Option<String>,
}

pub enum ChannelJoinType {
    Unspecified = 0,
    Room = 1,
    DirectMessage = 2,
    Group = 3,
}

#[derive(DeJson, SerJson, Debug, Clone, Default)]
pub struct ChannelJoin {
    pub hidden: bool,
    pub persistence: bool,
    pub target: String,
    #[nserde(rename = "type")]
    // TODO: Make ChannelJoinType
    pub channel_type: i32,
}

#[derive(DeJson, SerJson, Debug, Clone, Default)]
pub struct ChannelLeave {
    pub channel_id: String
}

#[derive(DeJson, SerJson, Debug, Clone, Default)]
pub struct ChannelMessageAck {
    pub channel_id: String,
    pub message_id: String,
    // TODO: What is the code?
    pub code: i32,
    pub username: String,
    pub create_time: Timestamp,
    pub update_time: Timestamp,
    pub persistent: bool,
    pub room_name: Option<String>,
    pub group_id: Option<String>,
    pub user_id_one: Option<String>,
    pub user_id_two: Option<String>,
}

#[derive(DeJson, SerJson, Debug, Clone, Default)]
pub struct ChannelMessageSend {
    pub channel_id: String,
    pub content: String,
}

#[derive(DeJson, SerJson, Debug, Clone, Default)]
pub struct ChannelMesageUpdate {
    pub channel_id: String,
    pub message_id: String,
    pub content: String
}

#[derive(DeJson, SerJson, Debug, Clone, Default)]
pub struct ChannelMesageRemove {
    pub channel_id: String,
    pub message_id: String,
}

#[derive(DeJson, SerJson, Debug, Clone, Default)]
pub struct ChannelPresenceEvent {
    pub channel_id: String,
    pub joins: Vec<UserPresence>,
    pub leaves: Vec<UserPresence>,
    pub room_name: Option<String>,
    pub group_id: Option<String>,
    pub user_id_one: Option<String>,
    pub user_id_two: Option<String>,
}

pub enum ErrorCode {
    RuntimeException = 0,
    UnrecognizedPayload = 1,
    MissingPayload = 2,
    BadInput = 3,
    MatchNotFound = 4,
    MatchJoinRejected = 5,
    RuntimeFunctionNotFound = 6,
    RuntimeFunctionException = 7,
}

#[derive(DeJson, SerJson, Debug, Clone, Default)]
pub struct Error {
    // TODO: Use ErrorCode
    pub code: i32,
    pub message: String,
    pub context: HashMap<String, String>,
}

#[derive(DeJson, SerJson, Debug, Clone, Default)]
pub struct Match {
    pub match_id: String,
    pub authoritative: bool,
    pub label: String,
    pub size: i32,
    pub presences: Vec<UserPresence>,
    #[nserde(rename = "self")]
    pub _self: UserPresence,
}

#[derive(DeJson, SerJson, Debug, Clone, Default)]
pub struct MatchCreate {
}

#[derive(DeJson, SerJson, Debug, Clone, Default)]
pub struct MatchData {
    pub match_id: String,
    pub presence: UserPresence,
    pub op_code: i64,
    pub data: Vec<u8>,
    pub reliable: bool,
}

#[derive(DeJson, SerJson, Debug, Clone, Default)]
pub struct MatchDataSend {
    pub match_id: String,
    pub op_code: i64,
    pub data: Vec<u8>,
    pub presences: UserPresence,
    pub reliable: bool,
}

#[derive(DeJson, SerJson, Debug, Clone, Default)]
pub struct MatchJoin {
    pub match_id: Option<String>,
    pub token: Option<String>,
    pub metadata: HashMap<String, String>,
}

#[derive(DeJson, SerJson, Debug, Clone, Default)]
pub struct MatchLeave {
    pub match_id: String,
}

#[derive(DeJson, SerJson, Debug, Clone, Default)]
pub struct MatchPresenceEvent {
    pub match_id: String,
    pub joins: Vec<UserPresence>,
    pub leaves: Vec<UserPresence>
}

#[derive(DeJson, SerJson, Debug, Clone, Default)]
pub struct MatchmakerAdd {
    pub min_count: i32,
    pub max_count: i32,
    pub query: String,
    pub string_properties: HashMap<String, String>,
    pub numeric_properties: HashMap<String, f64>,
}

#[derive(DeJson, SerJson, Debug, Clone, Default)]
pub struct MatchmakerUser {
    pub presence: UserPresence,
    pub party_id: String,
    pub string_properties: HashMap<String, String>,
    pub numeric_properties: HashMap<String, f64>,
}

#[derive(DeJson, SerJson, Debug, Clone, Default)]
pub struct MatchmakerMatched {
    pub ticket: String,
    pub match_id: Option<String>,
    pub token: Option<String>,
    pub users: Vec<MatchmakerUser>,
    #[nserde(rename = "self")]
    pub _self: MatchmakerUser,
}

#[derive(DeJson, SerJson, Debug, Clone, Default)]
pub struct MatchmakerRemove {
    pub ticket: String,
}

#[derive(DeJson, SerJson, Debug, Clone, Default)]
pub struct MatchmakerTicket {
    pub ticket: String,
}

#[derive(DeJson, SerJson, Debug, Clone, Default)]
pub struct Notifications {
    pub notifications: Vec<ApiNotification>,
}

#[derive(DeJson, SerJson, Debug, Clone, Default)]
pub struct Party {
    pub party_id: String,
    pub open: bool,
    pub max_size: i32,
    #[nserde(rename = "self")]
    pub _self: UserPresence,
    pub leader: UserPresence,
    pub presences: Vec<UserPresence>,
}

#[derive(DeJson, SerJson, Debug, Clone, Default)]
pub struct PartyCreate {
    pub open: bool,
    pub max_size: i32,
}

#[derive(DeJson, SerJson, Debug, Clone, Default)]
pub struct PartyJoin {
    pub party_id: String
}

#[derive(DeJson, SerJson, Debug, Clone, Default)]
pub struct PartyLeave {
    pub party_id: String
}

#[derive(DeJson, SerJson, Debug, Clone, Default)]
pub struct PartyPromote {
    pub party_id: String,
    pub presence: UserPresence,
}

#[derive(DeJson, SerJson, Debug, Clone, Default)]
pub struct PartyLeader {
    pub party_id: String,
    pub presence: UserPresence,
}

#[derive(DeJson, SerJson, Debug, Clone, Default)]
pub struct PartyAccept {
    pub party_id: String,
    pub presence: UserPresence,
}

#[derive(DeJson, SerJson, Debug, Clone, Default)]
pub struct PartyRemove {
    pub party_id: String,
    pub presence: UserPresence,
}

#[derive(DeJson, SerJson, Debug, Clone, Default)]
pub struct PartyClose {
    pub party_id: String,
}

#[derive(DeJson, SerJson, Debug, Clone, Default)]
pub struct PartyJoinRequestList {
    pub party_id: String,
}

#[derive(DeJson, SerJson, Debug, Clone, Default)]
pub struct PartyJoinRequest {
    pub party_id: String,
    pub presences: Vec<UserPresence>,
}

#[derive(DeJson, SerJson, Debug, Clone, Default)]
pub struct PartyMatchmakerAdd {
    pub party_id: String,
    pub min_count: i32,
    pub max_count: i32,
    pub query: String,
    pub string_properties: HashMap<String, String>,
    pub numeric_properties: HashMap<String, f64>,
}

#[derive(DeJson, SerJson, Debug, Clone, Default)]
pub struct PartyMatchmakerRemove {
    pub party_id: String,
    pub ticket: String,
}

#[derive(DeJson, SerJson, Debug, Clone, Default)]
pub struct PartyMatchmakerTicket {
    pub party_id: String,
    pub ticket: String,
}

#[derive(DeJson, SerJson, Debug, Clone, Default)]
pub struct PartyData {
    pub party_id: String,
    pub presence: UserPresence,
    pub op_code: i64,
    pub data: Vec<u8>,
}

#[derive(DeJson, SerJson, Debug, Clone, Default)]
pub struct PartyDataSend {
    pub party_id: String,
    pub op_code: i64,
    pub data: Vec<u8>,
}

#[derive(DeJson, SerJson, Debug, Clone, Default)]
pub struct PartyPresenceEvent {
    pub party_id: String,
    pub joins: Vec<UserPresence>,
    pub leaves: Vec<UserPresence>,
}

#[derive(DeJson, SerJson, Debug, Clone, Default)]
pub struct Ping {
}

#[derive(DeJson, SerJson, Debug, Clone, Default)]
pub struct Pong {
}

#[derive(DeJson, SerJson, Debug, Clone, Default)]
pub struct Status {
    pub presences: Vec<UserPresence>
}

#[derive(DeJson, SerJson, Debug, Clone, Default)]
pub struct StatusFollow {
    pub user_ids: Vec<String>,
    pub usernames: Vec<String>,
}

#[derive(DeJson, SerJson, Debug, Clone, Default)]
pub struct StatusPresenceEvent {
    pub joins: Vec<UserPresence>,
    pub leaves: Vec<UserPresence>,
}

#[derive(DeJson, SerJson, Debug, Clone, Default)]
pub struct StatusUnfollow {
    pub user_ids: Vec<String>,
}

#[derive(DeJson, SerJson, Debug, Clone, Default)]
pub struct StatusUpdate {
    pub status: String,
}

#[derive(DeJson, SerJson, Debug, Clone, Default)]
pub struct Stream {
    pub mode: i32,
    pub subject: String,
    pub subcontext: String,
    pub label: String,
}

#[derive(DeJson, SerJson, Debug, Clone, Default)]
pub struct StreamData {
    pub stream: Stream,
    pub sender: UserPresence,
    pub data: String,
    pub reliable: bool
}

#[derive(DeJson, SerJson, Debug, Clone, Default)]
pub struct StreamPresenceEvent {
    pub stream: Stream,
    pub joins: Vec<UserPresence>,
    pub leaves: Vec<UserPresence>,
}

#[derive(DeJson, SerJson, Debug, Clone, Default)]
pub struct UserPresence {
    pub persistence: bool,
    pub session_id: String,
    pub status: String,
    pub username: String,
    pub user_id: String,
}

#[derive(DeJson, SerJson, Debug, Clone, Default)]
pub struct WebSocketMessageEnvelope {
    pub cid: Option<String>,
    pub channel: Option<Channel>,
    pub channel_join: Option<ChannelJoin>,
    pub channel_leave: Option<ChannelLeave>,
    pub channel_message: Option<ApiChannelMessage>,
    pub channel_message_ack: Option<ChannelMessageAck>,
    pub channel_message_remove: Option<ChannelMesageRemove>,
    pub channel_message_send: Option<ChannelMessageSend>,
    pub channel_message_update: Option<ChannelMesageUpdate>,
    pub error: Option<Error>,
    pub matchmaker_add: Option<MatchmakerAdd>,
    pub matchmaker_matched: Option<MatchmakerMatched>,
    pub matchmaker_remove: Option<MatchmakerRemove>,
    pub matchmaker_ticket: Option<MatchmakerTicket>,
    #[nserde(rename = "match")]
    pub new_match: Option<Match>,
    pub match_create: Option<MatchCreate>,
    pub match_join: Option<MatchJoin>,
    pub match_leave: Option<MatchLeave>,
    pub match_presence_event: Option<MatchPresenceEvent>,
    pub match_data: Option<MatchData>,
    pub match_data_send: Option<MatchDataSend>,
    pub notifications: Option<ApiNotificationList>,
    pub rpc: Option<ApiRpc>,
    pub status: Option<Status>,
    pub status_follow: Option<StatusFollow>,
    pub status_presence_event: Option<StatusPresenceEvent>,
    pub status_unfollow: Option<StatusUnfollow>,
    pub status_update: Option<StatusUpdate>,
    pub stream_presence_event: Option<StreamPresenceEvent>,
    pub stream_data: Option<StreamData>,
    pub party: Option<Party>,
    pub party_create: Option<PartyCreate>,
    pub party_join: Option<PartyJoin>,
    pub party_leave: Option<PartyLeave>,
    pub party_promote: Option<PartyPromote>,
    pub party_leader: Option<PartyLeader>,
    pub party_accept: Option<PartyAccept>,
    pub party_remove: Option<PartyRemove>,
    pub party_close: Option<PartyClose>,
    pub party_join_request_list: Option<PartyJoinRequestList>,
    pub party_join_request: Option<PartyJoinRequest>,
    pub party_matchmaker_add: Option<PartyMatchmakerAdd>,
    pub party_matchmaker_remove: Option<PartyMatchmakerRemove>,
    pub party_matchmaker_ticket: Option<PartyMatchmakerTicket>,
    pub party_data: Option<PartyData>,
    pub party_data_send: Option<PartyDataSend>,
    pub party_presence_event: Option<PartyPresenceEvent>,
}

#[async_trait]
pub trait Socket {
    // It would make sense to have a future here
    fn on_closed<T>(&mut self, callback: T)
    where
        T: Fn() + 'static;

    fn on_connected<T>(&mut self, callback: T)
    where
        T: Fn() + 'static;

    fn on_received_channel_message<T>(&mut self, callback: T)
    where
        T: Fn(ApiChannelMessage) + 'static;

    async fn connect(&self, session: &mut Session, appear_online: bool, connect_timeout: i32);

    async fn close(&self);

    async fn create_match(&self) -> Match;

    async fn write_chat_message(&self, channel_id: &str, content: &str);

    async fn join_chat(
        &self,
        room_name: &str,
        channel_type: i32,
        persistence: bool,
        hidden: bool,
    ) -> Channel;
}
