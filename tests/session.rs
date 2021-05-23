use futures::executor::block_on;
use nakama_rs::api::ApiAccountDevice;
use nakama_rs::client::Client;
use nakama_rs::default_client::DefaultClient;
use std::collections::HashMap;

#[test]
fn test_session_variables() {
    let client = DefaultClient::new_with_adapter();

    let result = block_on(async {
        let mut vars = HashMap::new();
        vars.insert("ident".to_owned(), "hidden".to_owned());
        let mut session = client
            .authenticate_device("somedeviceid", None, true, vars)
            .await?;

        client.get_account(&mut session).await
    });

    println!("Result: {:?}", result);
    let account = result.unwrap();
    assert_eq!(
        account.devices[0].vars.get("ident"),
        Some(&"hidden".to_owned())
    );
}
