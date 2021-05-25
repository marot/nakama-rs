use futures::executor::block_on;
use nakama_rs::socket::Socket;

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
