use rand::Rng;
use std::env;
use std::thread::sleep;
use std::time::Duration;

use strm_privacy_driver::error::Error;
use strm_privacy_driver::test::demo::{DemoEvent, StrmMeta};
use strm_privacy_driver::{StrmPrivacyClient, StrmStatusCode};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        panic!("Wrong amount of arguments")
    }

    let client_id = args.get(1).unwrap();
    let client_secret = args.get(2).unwrap();

    println!("initializing client");
    let mut strm_privacy_client =
        StrmPrivacyClient::default(client_id.to_string(), client_secret.to_string()).await?;

    println!("sending events..");
    loop {
        let event = create_event();

        // catch specific status_codes and decide what to do
        match strm_privacy_client.send_event(event).await? {
            (StrmStatusCode::NO_CONTENT, _) => {
                println!("Event sent: {}", StrmStatusCode::NO_CONTENT)
            }
            (status_code, message) => {
                println!("{} {}", status_code, message)
            }
        }

        let duration = Duration::from_millis(500);
        sleep(duration)
    }
}

// create new event based on the example schema
fn create_event() -> DemoEvent {
    let consent_level = rand::thread_rng().gen_range(0..4);
    let uuid = Uuid::new_v4();

    DemoEvent {
        strm_meta: StrmMeta {
            event_contract_ref: "strmprivacy/example/1.3.0".to_string(),
            nonce: None,
            timestamp: None,
            key_link: None,
            billing_id: None,
            consent_levels: vec![consent_level],
        },
        unique_identifier: Some(uuid.to_string()),
        consistent_value: "consistent".to_string(),
        some_sensitive_value: Some("sensitive".to_string()),
        not_sensitive_value: Some("not sensitive".to_string()),
    }
}
