use pulsar::{
    message::{proto::command_subscribe::SubType, Payload},
    Consumer, DeserializeMessage, Pulsar, TokioExecutor,
};
use futures::TryStreamExt;
use serde::{Deserialize, Serialize};
use std::env;
use std::result::Result;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct TestData {
    data: String,
}

impl DeserializeMessage for TestData {
    type Output = Result<TestData, serde_json::Error>;

    fn deserialize_message(payload: &Payload) -> Self::Output {
        serde_json::from_slice(&payload.data)
    }
}

#[test]
fn serde_the_testdata() {
    let p = "{data:'this is a payload'}";
    assert_eq!(TestData::deserialize(p), TestData{data: "this is a payload".to_string()});
}


#[tokio::main]
async fn main() -> Result<(), pulsar::Error> {
    env_logger::init();
    
    let addr = env::var("PULSAR_ADDRESS")
        .ok()
        .unwrap_or_else(|| "pulsar://127.0.0.1:6650".to_string());
    let topic = env::var("PULSAR_TOPIC")
        .ok()
        .unwrap_or_else(|| "my-topic".to_string());

    let pulsar:Pulsar<_> = Pulsar::builder(addr, TokioExecutor).build().await?;

    let mut consumer: Consumer<TestData, _> = pulsar
        .consumer()
        .with_topic(topic)
        .with_consumer_name("my-consumer")
        .with_subscription_type(SubType::Exclusive)
        .with_subscription("my-sub")
        .build()
        .await?;

    let mut counter: usize = 0;

    while let Some(msg) = consumer.try_next().await? {
        consumer.ack(&msg).await?;
        log::info!("metadata: {:?}", msg.metadata());
        log::info!("id: {:?}", msg.message_id());

        let test_data = match msg.deserialize() {
            Ok(data) => data,
            Err(e) => {
                log::error!("could not deserialize message: {:?}", e);
                // break;
                TestData{data: "FAILURE".to_string()}
            }
        };

        if test_data.data.as_str() == "FAILURE" {
            log::error!("Unexpected payload: {}", &test_data.data);
        }
        counter += 1;
        log::info!("received {} messages", counter);
        println!("data: {:?}", test_data);

    }

    
    println!("EXITING");
    Ok(())
}


