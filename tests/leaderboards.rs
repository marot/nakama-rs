use futures::executor::block_on;
use nakama_rs::client::Client;

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