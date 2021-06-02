use futures::executor::block_on;
use nakama_rs::client::Client;
use nakama_rs::api::ApiOverrideOperator;

mod helpers;

#[test]
fn test_write_leaderboard_record() {
    block_on(async {
        let (client, mut session) = helpers::authenticated_client("leaderboardclient1").await;
        let result = client.write_leaderboard_record(&mut session, "wins", 1, None, None, None).await;
        println!("{:?}", result);
        assert_eq!(result.is_ok(), true);
    });
}

#[test]
fn test_write_leaderboard_record_subscore_and_override_operator() {
    block_on(async {
        let (client, mut session) = helpers::authenticated_client("leaderboardclient1").await;
        let result = client.write_leaderboard_record(&mut session, "wins", 1, Some(50), Some(ApiOverrideOperator::SET), None).await;
        println!("{:?}", result);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap().subscore, Some("50".to_owned()));
    });
}

#[test]
fn test_delete_leaderboard_record() {
    block_on(async {
        let (client, mut session) = helpers::authenticated_client("leaderboardclient1").await;
        client.write_leaderboard_record(&mut session, "wins", 1, Some(50), Some(ApiOverrideOperator::SET), None).await;
        let result = client.delete_leaderboard_record(&mut session, "wins").await;
        println!("{:?}", result);
        assert_eq!(result.is_ok(), true);
    });
}

#[test]
fn test_list_leaderboard_records() {
    block_on(async {
        let (client, mut session) = helpers::authenticated_client("leaderboardclient1").await;
        let (client2, mut session2) = helpers::authenticated_client("leaderboardclient2").await;
        client.write_leaderboard_record(&mut session, "wins", 1, Some(50), Some(ApiOverrideOperator::SET), None).await;
        client.write_leaderboard_record(&mut session2, "wins", 2, Some(50), Some(ApiOverrideOperator::SET), None).await;
        let result1 = client.list_leaderboard_records(&mut session, "wins", &[], None, Some(1), None).await.unwrap();
        let result2 = client.list_leaderboard_records(&mut session, "wins", &[], None, None, Some(&result1.next_cursor)).await.unwrap();
        println!("{:?}", result2);
        assert_eq!(result1.prev_cursor.is_empty(), true);
        assert_eq!(result2.records.len() >= 1, true);
        assert_eq!(result2.next_cursor.is_empty(), true);
    });
}

#[test]
fn test_list_leaderboard_records_around_owner() {
    block_on(async {
        let (client, mut session) = helpers::authenticated_client("leaderboardclient1").await;
        let (client2, mut session2) = helpers::authenticated_client("leaderboardclient2").await;
        client.write_leaderboard_record(&mut session, "wins", 1, Some(50), Some(ApiOverrideOperator::SET), None).await;
        client.write_leaderboard_record(&mut session2, "wins", 2, Some(50), Some(ApiOverrideOperator::SET), None).await;
        let user_id = client.get_account(&mut session).await.unwrap().user.id;
        let result = client.list_leaderboard_records_around_owner(&mut session, "wins", &user_id, None, Some(1)).await.unwrap();
        println!("{:?}", result);
        assert_eq!(result.records.len() >= 1, true);
    });
}