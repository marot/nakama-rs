mod helpers;

use nakama_rs::client::Client;
use futures::executor::block_on;

#[test]
fn test_create_group() {
    block_on(async {
        let (client, mut session1, _, _) = helpers::clients_with_users("friendtestuser1", "friendtestuser2", "friendtestuser3").await;
        helpers::remove_group_if_exists(&client, &mut session1, "MyGroup").await;
        let result = client.create_group(&mut session1, "MyGroup", None, None, None, Some(true), None).await;
        println!("{:?}", result);
        assert_eq!(result.is_ok(), true);
    });
}

#[test]
fn test_update_group() {
    block_on(async {
        let (client, mut session1, _, _) = helpers::clients_with_users("friendtestuser1", "friendtestuser2", "friendtestuser3").await;
        let group = helpers::re_create_group(&client, &mut session1, "UpdateGroup").await;
        helpers::remove_group_if_exists(&client, &mut session1, "AnUpdateGroup").await;
        let result = client.update_group(&mut session1, &group.id, "AnUpdateGroup", false, Some("MyDescription"), Some("https://avatar.url"), Some("en")).await;
        // TODO: Changing the name of a group to an already taken name triggers a 500 error
        // let result = client.update_group(&mut session1, &group.id, "MyUpdateGroup", false, Some("MyDescription"), Some("https://avatar.url"), Some("en")).await;
        println!("{:?}", result);
        assert_eq!(result.is_ok(), true);
    });
}

#[test]
fn test_add_group_users() {
    block_on(async {
        let (client, mut session1, mut session2, mut session3) = helpers::clients_with_users("friendtestuser1", "friendtestuser2", "friendtestuser3").await;
        let group = helpers::re_create_group(&client, &mut session1, "AddGroupUsers").await;
        let account2 = client.get_account(&mut session2).await.unwrap();
        let account3 = client.get_account(&mut session3).await.unwrap();
        let result = client.add_group_users(&mut session1, &group.id, &[&account2.user.id, &account3.user.id]).await;
        println!("{:?}", result);
        assert_eq!(result.is_ok(), true);
    });
}

#[test]
fn test_ban_group_users() {
    block_on(async {
        let (client, mut session1, mut session2, _) = helpers::clients_with_users("friendtestuser1", "friendtestuser2", "friendtestuser3").await;
        let group = helpers::re_create_group(&client, &mut session1, "BanGroupUsers").await;
        let account2 = client.get_account(&mut session2).await.unwrap();
        let result = client.ban_group_users(&mut session1, &group.id, &[&account2.user.id]).await;
        println!("{:?}", result);
        assert_eq!(result.is_ok(), true);
    });
}

#[test]
fn test_delete_group() {
    block_on(async {
        let (client, mut session1, mut session2, _) = helpers::clients_with_users("friendtestuser1", "friendtestuser2", "friendtestuser3").await;
        let group = helpers::re_create_group(&client, &mut session1, "DeleteGroup").await;
        let result = client.delete_group(&mut session1, &group.id).await;
        println!("{:?}", result);
        assert_eq!(result.is_ok(), true);
    });

}

#[test]
fn test_promote_group_user() {
    block_on(async {
        let (client, mut session1, mut session2, _) = helpers::clients_with_users("friendtestuser1", "friendtestuser2", "friendtestuser3").await;
        let group = helpers::re_create_group(&client, &mut session1, "PromoteGroupUser").await;
        let account2 = client.get_account(&mut session2).await.unwrap();
        let result = client.promote_group_user(&mut session1, &group.id, &[&account2.user.id]).await;
        println!("{:?}", result);
        assert_eq!(result.is_ok(), true);
    });
}

#[test]
fn test_demote_group_users() {
    block_on(async {
        let (client, mut session1, mut session2, _) = helpers::clients_with_users("friendtestuser1", "friendtestuser2", "friendtestuser3").await;
        let group = helpers::re_create_group(&client, &mut session1, "DemoteGroupUser").await;
        let account2 = client.get_account(&mut session2).await.unwrap();
        client.promote_group_user(&mut session1, &group.id, &[&account2.user.id]).await.unwrap();
        let result = client.demote_group_users(&mut session1, &group.id, &[&account2.user.id]).await;
        println!("{:?}", result);
        assert_eq!(result.is_ok(), true);
    });
}

#[test]
fn test_join_group() {
    block_on(async {
        let (client, mut session1, mut session2, _) = helpers::clients_with_users("friendtestuser1", "friendtestuser2", "friendtestuser3").await;
        let group = helpers::re_create_group(&client, &mut session1, "JoinGroup").await;
        let result = client.join_group(&mut session2, &group.id).await;
        println!("{:?}", result);
        assert_eq!(result.is_ok(), true);
    });
}

#[test]
fn test_kick_group_users() {
    block_on(async {
        let (client, mut session1, mut session2, _) = helpers::clients_with_users("friendtestuser1", "friendtestuser2", "friendtestuser3").await;
        let group = helpers::re_create_group(&client, &mut session1, "KickGroupUsers").await;
        let account2 = client.get_account(&mut session2).await.unwrap();
        client.join_group(&mut session2, &group.id).await.unwrap();
        let result = client.kick_group_users(&mut session1, &group.id, &[&account2.user.id]).await;
        println!("{:?}", result);
        assert_eq!(result.is_ok(), true);
    });
}

#[test]
fn test_leave_group() {
    block_on(async {
        let (client, mut session1, mut session2, _) = helpers::clients_with_users("friendtestuser1", "friendtestuser2", "friendtestuser3").await;
        let group = helpers::re_create_group(&client, &mut session1, "LeaveGroup").await;
        client.join_group(&mut session2, &group.id).await.unwrap();
        let result = client.leave_group(&mut session2, &group.id).await;
        println!("{:?}", result);
        assert_eq!(result.is_ok(), true);
    });

}

#[test]
fn test_list_group_users() {

}

#[test]
fn test_list_groups() {

}

#[test]
fn test_list_current_user_groups() {

}

#[test]
fn test_list_user_groups() {

}

