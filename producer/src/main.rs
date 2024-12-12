use rdkafka::config::ClientConfig;
use rdkafka::consumer::StreamConsumer;
use rdkafka::message::{Header, OwnedHeaders};
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::get_rdkafka_version;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let (version_n, version_s) = get_rdkafka_version();
    println!("version num: 0x{:08x}, {}", version_n, version_s);

    let producer: &FutureProducer = &ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");

    let futures = (0..5)
        .map(|i| async move {
            let delivery_status = producer
                .send(
                    FutureRecord::to("test")
                        .payload(&format!("Message: {}", i))
                        .key(&format!("Key {}", i))
                        .headers(OwnedHeaders::new().insert(Header {
                            key: "header_key",
                            value: Some("header_value"),
                        })), // record to send
                    Duration::from_secs(0), // queue timeout
                )
                .await;

            println!("Delivery status for message {} received", i);
            delivery_status
        })
        .collect::<Vec<_>>();

    for future in futures {
        println!("Future completed. Result: {:?}", future.await);
    }
}
