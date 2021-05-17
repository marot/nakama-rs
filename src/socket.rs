use crate::api::ApiChannelMessage;
use crate::session::Session;
use crate::socket_adapter::SocketAdapter;
use async_trait::async_trait;
use nanoserde::{DeJson, SerJson};

#[derive(DeJson, SerJson, Debug, Clone, Default)]
pub struct MatchCreate {
}

#[derive(DeJson, SerJson, Debug, Clone, Default)]
pub struct Match {
    match_id: String,
}

#[derive(DeJson, SerJson, Debug, Clone, Default)]
pub struct Channel {
    id: String,
    // presences
    // self
    room_name: String,
    group_id: String,
    user_id_one: String,
    user_id_two: String,
}

#[derive(DeJson, SerJson, Debug, Clone, Default)]
pub struct ChannelJoinMessage {
    pub hidden: bool,
    pub persistence: bool,
    pub target: String,
    #[nserde(rename = "channel")]
    pub channel_type: i32,
}


#[derive(DeJson, SerJson, Debug, Clone, Default)]
pub struct WebSocketMessageEnvelope {
    pub cid: Option<String>,
    pub channel: Option<Channel>,
    pub channel_join: Option<ChannelJoinMessage>,
    // pub channel_leave: Option<ChannelLeaveMessage>,
    // pub channel_message: Option<ApiChannelMessage>,
    // pub channel_message_ack: Option<ChannelMessageAck>,
    // pub channel_message_remove: Option<ChannelRemoveMessage>,
    // pub channel_message_send: Option<ChannelSendMessage>,
    // pub channel_message_update: Option<ChannelUpdateMessage>,
    // pub error: Option<WebSocketErrorMessage>,
    // pub matchmaker_add: Option<MatchmakerAddMessage>,
    // pub matchmaker_matched: Option<MatchmakerMatched>,
    // pub matchmaker_remove: Option<MatchmakerRemoveMessage>,
    // pub matchmaker_ticket: Option<MatchmakerTicket>,
    #[nserde(rename = "match")]
    pub new_match: Option<Match>,
    pub match_create: Option<MatchCreate>,
    // pub match_join: Option<MatchJoin>,
    // pub match_leave: Option<MatchLeave>,
    // pub match_presence_event: Option<MatchPresenceEvent>,
    // pub match_data: Option<MatchState>,
    // pub match_data_send: Option<MatchStateSend>,
    // pub notifications: Option<ApiNotificationList>,
    // pub rpc: Option<ApiRpc>,
    // pub status: Option<Status>,
    // pub status_follow: Option<StatusFollowMessage>,
    // pub status_presence_event: Option<StatusPresenceEvent>,
    // pub status_unfollow: Option<StatusUnfollowMessage>,
    // pub status_update: Option<StatusUnfollowMessage>,
    // pub stream_presence_event: Option<StreamPresenceEvent>,
    // pub stream_data: Option<StreamState>,
    // pub party: Option<Party>,
    // pub party_create: Option<PartyCreate>,
    // pub party_join: Option<PartyJoin>,
    // pub party_leave: Option<PartyLeave>,
    // pub party_promote: Option<PartyPromote>,
    // pub party_leader: Option<PartyLeader>,
    // pub party_accept: Option<PartyAccept>,
    // pub party_remove: Option<PartyMemberRemove>,
    // pub party_close: Option<PartyClose>,
    // pub party_join_request_list: Option<PartyJoinRequestList>,
    // pub party_join_request: Option<PartyJoinRequest>,
    // pub party_matchmaker_add: Option<PartyMatchmakerAdd>,
    // pub party_matchmaker_remove: Option<PartyMatchmakerRemove>,
    // pub party_matchmaker_ticket: Option<PartyMatchmakerTicket>,
    // pub party_data: Option<PartyData>,
    // pub party_data_send: Option<PartyDataSend>,
    // pub party_presence_event: Option<PartyPresenceEvent>,
}


#[async_trait(?Send)]
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

    async fn connect(&mut self, session: &mut Session, appear_online: bool, connect_timeout: i32);

    async fn close(&mut self);

    async fn create_match(&self) -> Match;

    async fn join_chat(&self, room_name: &str, channel_type: i32, persistence: bool, hidden: bool) -> Channel;
}
