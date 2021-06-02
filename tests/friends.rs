use futures::executor::block_on;
use nakama_rs::client::Client;
use nakama_rs::test_helpers;

#[test]
fn test_add_friend_username() {
    block_on(async {
        let (client, mut session1, _, _) = test_helpers::clients_with_users(
            "friendtestuser1",
            "friendtestuser2",
            "friendtestuser3",
        )
        .await;
        let result = client
            .add_friends(&mut session1, &[], &["friendtestuser2"])
            .await;
        println!("{:?}", result);
        assert_eq!(result.is_ok(), true);
    });
}

#[test]
fn test_add_friend_id() {
    block_on(async {
        let (client, mut session1, mut session2, _) = test_helpers::clients_with_users(
            "friendtestuser1",
            "friendtestuser2",
            "friendtestuser3",
        )
        .await;
        let account2 = client.get_account(&mut session2).await.unwrap();
        let result = client
            .add_friends(&mut session1, &[&account2.user.id], &[])
            .await;
        println!("{:?}", result);
        assert_eq!(result.is_ok(), true);
    });
}

#[test]
fn test_list_friend() {
    block_on(async {
        let (client, mut session1, _, _) = test_helpers::clients_with_users(
            "friendtestuser1",
            "friendtestuser2",
            "friendtestuser3",
        )
        .await;
        client
            .add_friends(&mut session1, &[], &["friendtestuser2", "friendtestuser3"])
            .await
            .unwrap();
        let friends = client
            .list_friends(&mut session1, None, Some(1), None)
            .await
            .unwrap();
        println!("{:?}", friends);
        assert_eq!(friends.friends.len(), 1);
        let friends = client
            .list_friends(&mut session1, None, Some(1), Some(&friends.cursor))
            .await
            .unwrap();
        println!("{:?}", friends);
        assert_eq!(friends.friends.len(), 1);
        assert_eq!(friends.cursor.is_empty(), true);
    });
}

#[test]
fn test_delete_friend() {
    block_on(async {
        let (client, mut session1, _, _) = test_helpers::clients_with_users(
            "friendtestuser1",
            "friendtestuser2",
            "friendtestuser3",
        )
        .await;
        client
            .add_friends(&mut session1, &[], &["friendtestuser2"])
            .await
            .unwrap();
        let result = client
            .delete_friends(&mut session1, &[], &["friendtestuser2"])
            .await;
        println!("{:?}", result);
        assert_eq!(result.is_ok(), true);
    });
}

#[test]
fn test_block_friend() {
    block_on(async {
        let (client, mut session1, _, _) = test_helpers::clients_with_users(
            "friendtestuser1",
            "friendtestuser2",
            "friendtestuser3",
        )
        .await;
        let result = client
            .block_friends(&mut session1, &[], &["friendtestuser2"])
            .await;
        println!("{:?}", result);
        assert_eq!(result.is_ok(), true);
    })
}
