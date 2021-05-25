use futures::executor::block_on;
use nakama_rs::socket::Socket;
use std::sync::mpsc;
use simple_logger::SimpleLogger;
use log::LevelFilter;
use std::thread::sleep;
use std::time::Duration;

mod helpers;

#[test]
fn create_and_close_party() {
    block_on(async {
        let (socket1, socket2, account1, account2) =
            helpers::sockets_with_users("partyuserone", "partyusertwo").await;

        let party = socket1.create_party(true, 2).await.unwrap();
        socket1.close_party(&party.party_id).await;
    })
}

#[test]
fn join_and_leave_party() {
    block_on(async {
        let (socket1, socket2, account1, account2) =
            helpers::sockets_with_users("partyuserone", "partyusertwo").await;

        let party = socket1.create_party(true, 2).await.unwrap();
        socket2.join_party(&party.party_id).await;
        socket2.leave_party(&party.party_id).await;
    })
}

#[test]
fn promote_and_remove_party_member() {

    block_on(async {
        let (tx, rx) = mpsc::channel();
        let (mut socket1, mut socket2, account1, account2) =
            helpers::sockets_with_users("partyuserone", "partyusertwo").await;

        socket1.on_received_party_presence(move |presence| {
            tx.send(presence);
        });

        let party = socket1.create_party(true, 2).await.unwrap();
        // Wait for first party presence event
        rx.recv();

        socket2.join_party(&party.party_id).await.unwrap();
        // Wait for joined user
        let mut joined_presence = rx.recv().unwrap();
        let user_presence = joined_presence.joins.remove(0);

        socket1.promote_party_member(&party.party_id, user_presence.clone()).await.unwrap();
        socket2.remove_party_member(&party.party_id, party.leader).await.unwrap();
    })
}

#[test]
fn test_private_group() {
    block_on(async {
        let (mut socket1, socket2, account1, account2) =
            helpers::sockets_with_users("partyuserone", "partyusertwo").await;

        let party = socket1.create_party(false, 2).await.unwrap();
        socket2.join_party(&party.party_id).await.unwrap();
        let mut join_requests = socket1.list_party_join_requests(&party.party_id).await.unwrap();
        socket1.accept_party_member(&party.party_id, &join_requests.presences[0]).await.unwrap();
        socket1.promote_party_member(&party.party_id,join_requests.presences.remove(0)).await.unwrap();
        socket2.remove_party_member(&party.party_id, party.leader).await.unwrap();
    });
}

#[test]
fn test_send_party_data() {
    SimpleLogger::new()
        .with_level(LevelFilter::Off)
        .with_module_level("nakama_rs", LevelFilter::Trace)
        .init()
        .unwrap();
    block_on(async {
        let (tx, rx) = mpsc::channel();
        let (mut socket1, mut socket2, account1, account2) =
            helpers::sockets_with_users("partyuserone", "partyusertwo").await;

        let party = socket1.create_party(true, 2).await.unwrap();
        socket2.join_party(&party.party_id).await.unwrap();

        socket2.on_received_party_data(move |data| {
            tx.send(data);
        });
        socket1.send_party_data(&party.party_id, 1, &[1,2,3,4]).await;

        println!("{:?}", rx.recv());
    })
}
