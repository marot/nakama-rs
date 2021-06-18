// Copyright 2021 The Nakama Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::api::{
    ApiChannelMessageList, ApiDeleteStorageObjectId, ApiFriendList, ApiGroup, ApiGroupList,
    ApiGroupUserList, ApiLeaderboardRecord, ApiLeaderboardRecordList, ApiMatchList,
    ApiNotificationList, ApiOverrideOperator, ApiReadStorageObjectId, ApiRpc, ApiStorageObjectAcks,
    ApiStorageObjectList, ApiStorageObjects, ApiTournamentList, ApiTournamentRecordList,
    ApiUserGroupList, ApiUsers, ApiValidatePurchaseResponse, ApiWriteStorageObject,
};
use crate::api_gen::ApiAccount;
use crate::session::Session;
use async_trait::async_trait;
use std::collections::HashMap;
use std::error::Error;

#[async_trait]
pub trait Client {
    type Error: Error;

    async fn add_friends(
        &self,
        session: &Session,
        ids: &[&str],
        usernames: &[&str],
    ) -> Result<(), Self::Error>;

    async fn add_group_users(
        &self,
        session: &Session,
        group_id: &str,
        ids: &[&str],
    ) -> Result<(), Self::Error>;

    async fn authenticate_apple(
        &self,
        token: &str,
        username: Option<&str>,
        create: bool,
        vars: HashMap<&str, &str>,
    ) -> Result<Session, Self::Error>;

    async fn authenticate_custom(
        &self,
        id: &str,
        username: Option<&str>,
        create: bool,
        vars: HashMap<&str, &str>,
    ) -> Result<Session, Self::Error>;

    async fn authenticate_device(
        &self,
        id: &str,
        username: Option<&str>,
        create: bool,
        vars: HashMap<&str, &str>,
    ) -> Result<Session, Self::Error>;

    async fn authenticate_email(
        &self,
        email: &str,
        password: &str,
        username: Option<&str>,
        create: bool,
        vars: HashMap<&str, &str>,
    ) -> Result<Session, Self::Error>;

    async fn authenticate_facebook(
        &self,
        token: &str,
        username: Option<&str>,
        create: bool,
        vars: HashMap<&str, &str>,
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
        vars: HashMap<&str, &str>,
    ) -> Result<Session, Self::Error>;

    async fn authenticate_google(
        &self,
        token: &str,
        username: Option<&str>,
        create: bool,
        vars: HashMap<&str, &str>,
    ) -> Result<Session, Self::Error>;

    async fn authenticate_steam(
        &self,
        token: &str,
        username: Option<&str>,
        create: bool,
        vars: HashMap<&str, &str>,
    ) -> Result<Session, Self::Error>;

    async fn ban_group_users(
        &self,
        session: &Session,
        group_id: &str,
        user_ids: &[&str],
    ) -> Result<(), Self::Error>;

    async fn block_friends(
        &self,
        session: &Session,
        ids: &[&str],
        usernames: &[&str],
    ) -> Result<(), Self::Error>;

    async fn create_group(
        &self,
        session: &Session,
        name: &str,
        description: Option<&str>,
        avatar_url: Option<&str>,
        lang_tag: Option<&str>,
        open: Option<bool>,
        max_count: Option<i32>,
    ) -> Result<ApiGroup, Self::Error>;

    async fn delete_friends(
        &self,
        session: &Session,
        ids: &[&str],
        usernames: &[&str],
    ) -> Result<(), Self::Error>;

    async fn delete_group(&self, session: &Session, group_id: &str) -> Result<(), Self::Error>;

    async fn delete_leaderboard_record(
        &self,
        session: &Session,
        leaderboard_id: &str,
    ) -> Result<(), Self::Error>;

    async fn delete_notifications(
        &self,
        session: &Session,
        ids: &[&str],
    ) -> Result<(), Self::Error>;

    async fn delete_storage_objects(
        &self,
        session: &Session,
        ids: &[ApiDeleteStorageObjectId],
    ) -> Result<(), Self::Error>;

    async fn demote_group_users(
        &self,
        session: &Session,
        group_id: &str,
        user_ids: &[&str],
    ) -> Result<(), Self::Error>;

    async fn event(
        &self,
        session: &Session,
        name: &str,
        properties: HashMap<&str, &str>,
    ) -> Result<(), Self::Error>;

    async fn get_account(&self, session: &Session) -> Result<ApiAccount, Self::Error>;

    async fn get_users(
        &self,
        session: &Session,
        ids: &[&str],
        usernames: &[&str],
        facebook_ids: &[&str],
    ) -> Result<ApiUsers, Self::Error>;

    async fn import_facebook_friends(
        &self,
        session: &Session,
        token: &str,
        reset: Option<bool>,
    ) -> Result<(), Self::Error>;

    async fn import_steam_friends(
        &self,
        session: &Session,
        token: &str,
        reset: Option<bool>,
    ) -> Result<(), Self::Error>;

    async fn join_group(&self, session: &Session, group_id: &str) -> Result<(), Self::Error>;

    async fn join_tournament(
        &self,
        session: &Session,
        tournament_id: &str,
    ) -> Result<(), Self::Error>;

    async fn kick_group_users(
        &self,
        session: &Session,
        group_id: &str,
        ids: &[&str],
    ) -> Result<(), Self::Error>;

    async fn leave_group(&self, session: &Session, group_id: &str) -> Result<(), Self::Error>;

    async fn link_apple(&self, session: &Session, token: &str) -> Result<(), Self::Error>;

    async fn link_custom(&self, session: &Session, id: &str) -> Result<(), Self::Error>;

    async fn link_device(&self, session: &Session, id: &str) -> Result<(), Self::Error>;

    async fn link_email(
        &self,
        session: &Session,
        email: &str,
        password: &str,
    ) -> Result<(), Self::Error>;

    async fn link_facebook(
        &self,
        session: &Session,
        token: &str,
        import: Option<bool>,
    ) -> Result<(), Self::Error>;

    async fn link_game_center(
        &self,
        session: &Session,
        bundle_id: &str,
        player_id: &str,
        public_key_url: &str,
        salt: &str,
        signature: &str,
        timestamp: &str,
    ) -> Result<(), Self::Error>;

    async fn link_google(&self, session: &Session, token: &str) -> Result<(), Self::Error>;

    async fn link_steam(
        &self,
        session: &Session,
        token: &str,
        import: bool,
    ) -> Result<(), Self::Error>;

    async fn list_channel_messages(
        &self,
        session: &Session,
        channel_id: &str,
        limit: Option<i32>,
        forward: Option<bool>,
        cursor: Option<&str>,
    ) -> Result<ApiChannelMessageList, Self::Error>;

    async fn list_friends(
        &self,
        session: &Session,
        state: Option<i32>,
        limit: Option<i32>,
        cursor: Option<&str>,
    ) -> Result<ApiFriendList, Self::Error>;

    async fn list_group_users(
        &self,
        session: &Session,
        group_id: &str,
        state: Option<i32>,
        limit: Option<i32>,
        cursor: Option<&str>,
    ) -> Result<ApiGroupUserList, Self::Error>;

    async fn list_groups(
        &self,
        session: &Session,
        name: Option<&str>,
        limit: Option<i32>,
        cursor: Option<&str>,
    ) -> Result<ApiGroupList, Self::Error>;

    async fn list_leaderboard_records(
        &self,
        session: &Session,
        leaderboard_id: &str,
        owner_ids: &[&str],
        expiry: Option<&str>,
        limit: Option<i32>,
        cursor: Option<&str>,
    ) -> Result<ApiLeaderboardRecordList, Self::Error>;

    async fn list_leaderboard_records_around_owner(
        &self,
        session: &Session,
        leaderboard_id: &str,
        owner_id: &str,
        expiry: Option<&str>,
        limit: Option<i32>,
    ) -> Result<ApiLeaderboardRecordList, Self::Error>;

    async fn list_matches(
        &self,
        session: &Session,
        min: Option<i32>,
        max: Option<i32>,
        limit: Option<i32>,
        authoritative: Option<bool>,
        label: &str,
        query: &str,
    ) -> Result<ApiMatchList, Self::Error>;

    async fn list_notifications(
        &self,
        session: &Session,
        limit: Option<i32>,
        cacheable_cursor: Option<&str>,
    ) -> Result<ApiNotificationList, Self::Error>;

    async fn list_storage_objects(
        &self,
        session: &Session,
        collection: &str,
        limit: Option<i32>,
        cursor: Option<&str>,
    ) -> Result<ApiStorageObjectList, Self::Error>;

    async fn list_tournament_records_around_owner(
        &self,
        session: &Session,
        tournament_id: &str,
        owner_id: &str,
        expiry: Option<&str>,
        limit: Option<i32>,
    ) -> Result<ApiTournamentRecordList, Self::Error>;

    async fn list_tournament_records(
        &self,
        session: &Session,
        tournament_id: &str,
        owner_ids: &[&str],
        expiry: Option<&str>,
        limit: Option<i32>,
        cursor: Option<&str>,
    ) -> Result<ApiTournamentRecordList, Self::Error>;

    async fn list_tournaments(
        &self,
        session: &Session,
        category_start: Option<i32>,
        category_end: Option<i32>,
        start_time: Option<i32>,
        end_time: Option<i32>,
        limit: Option<i32>,
        cursor: Option<&str>,
    ) -> Result<ApiTournamentList, Self::Error>;

    async fn list_user_groups(
        &self,
        session: &Session,
        user_id: &str,
        state: Option<i32>,
        limit: Option<i32>,
        cursor: Option<&str>,
    ) -> Result<ApiUserGroupList, Self::Error>;

    async fn list_users_storage_objects(
        &self,
        session: &Session,
        collection: &str,
        user_id: &str,
        limit: Option<i32>,
        cursor: Option<&str>,
    ) -> Result<ApiStorageObjectList, Self::Error>;

    async fn promote_group_user(
        &self,
        session: &Session,
        group_id: &str,
        ids: &[&str],
    ) -> Result<(), Self::Error>;

    async fn read_storage_objects(
        &self,
        session: &Session,
        ids: &[ApiReadStorageObjectId],
    ) -> Result<ApiStorageObjects, Self::Error>;

    async fn rpc(
        &self,
        session: &Session,
        id: &str,
        payload: Option<&str>,
    ) -> Result<ApiRpc, Self::Error>;

    async fn session_logout(&self, session: &Session) -> Result<(), Self::Error>;

    async fn session_refresh(
        &self,
        session: &Session,
        vars: HashMap<&str, &str>,
    ) -> Result<(), Self::Error>;

    async fn unlink_apple(&self, session: &Session, token: &str) -> Result<(), Self::Error>;

    async fn unlink_custom(&self, session: &Session, id: &str) -> Result<(), Self::Error>;

    async fn unlink_device(&self, session: &Session, id: &str) -> Result<(), Self::Error>;

    async fn unlink_email(
        &self,
        session: &Session,
        email: &str,
        password: &str,
    ) -> Result<(), Self::Error>;

    async fn unlink_facebook(&self, session: &Session, token: &str) -> Result<(), Self::Error>;

    async fn unlink_game_center(
        &self,
        session: &Session,
        bundle_id: &str,
        player_id: &str,
        public_key_url: &str,
        salt: &str,
        signature: &str,
        timestamp: &str,
    ) -> Result<(), Self::Error>;

    async fn unlink_google(&self, session: &Session, token: &str) -> Result<(), Self::Error>;

    async fn unlink_steam(&self, session: &Session, token: &str) -> Result<(), Self::Error>;

    async fn update_account(
        &self,
        session: &Session,
        username: &str,
        display_name: Option<&str>,
        avatar_url: Option<&str>,
        lang_tag: Option<&str>,
        location: Option<&str>,
        timezone: Option<&str>,
    ) -> Result<(), Self::Error>;

    async fn update_group(
        &self,
        session: &Session,
        group_id: &str,
        name: &str,
        open: bool,
        description: Option<&str>,
        avatar_url: Option<&str>,
        lang_tag: Option<&str>,
    ) -> Result<(), Self::Error>;

    async fn validate_purchase_apple(
        &self,
        session: &Session,
        receipt: &str,
    ) -> Result<ApiValidatePurchaseResponse, Self::Error>;

    async fn validate_purchase_google(
        &self,
        session: &Session,
        receipt: &str,
    ) -> Result<ApiValidatePurchaseResponse, Self::Error>;

    async fn validate_purchase_huawei(
        &self,
        session: &Session,
        receipt: &str,
        signature: &str,
    ) -> Result<ApiValidatePurchaseResponse, Self::Error>;

    async fn write_leaderboard_record(
        &self,
        session: &Session,
        leaderboard_id: &str,
        score: i64,
        sub_score: Option<i64>,
        override_operator: Option<ApiOverrideOperator>,
        metadata: Option<&str>,
    ) -> Result<ApiLeaderboardRecord, Self::Error>;

    async fn write_storage_objects(
        &self,
        session: &Session,
        objects: &[ApiWriteStorageObject],
    ) -> Result<ApiStorageObjectAcks, Self::Error>;

    async fn write_tournament_record(
        &self,
        session: &Session,
        tournament_id: &str,
        score: i64,
        sub_score: Option<i64>,
        override_operator: Option<ApiOverrideOperator>,
        metadata: Option<&str>,
    ) -> Result<ApiLeaderboardRecord, Self::Error>;
}
