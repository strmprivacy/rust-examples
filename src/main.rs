use std::env;

use strm_privacy_driver::error::Error;
use strm_privacy_driver::{StrmPrivacyClient, StrmStatusCode};
use strmprivacy_schema_strmprivacy_demo::{DemoEvent, StrmMeta};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        panic!("Wrong amount of arguments")
    }

    let billing_id = args.get(1).unwrap();
    let client_id = args.get(2).unwrap();
    let client_secret = args.get(3).unwrap();

    let mut strm_privacy_client = StrmPrivacyClient::default(
        billing_id.to_string(),
        client_id.to_string(),
        client_secret.to_string(),
    )
    .await?;

    let event = create_event();

    // catch specific status_codes and decide what to do
    match strm_privacy_client.send_event(event).await? {
        (StrmStatusCode::NO_CONTENT, _) => {
            println!("Success!")
        }
        (status_code, message) => {
            println!("{} {}", status_code, message)
        }
    }

    Ok(())
}

// create new event based on the example schema
fn create_event() -> DemoEvent {
    DemoEvent {
        strm_meta: StrmMeta {
            event_contract_ref: "strmprivacy/example/1.3.0".to_string(),
            nonce: None,
            timestamp: None,
            key_link: None,
            billing_id: None,
            consent_levels: vec![0],
        },
        unique_identifier: Some("unique".to_string()),
        consistent_value: "consistent".to_string(),
        some_sensitive_value: Some("sensitive".to_string()),
        not_sensitive_value: Some("not sensitive".to_string()),
    }
}
