use rdkafka::client::ClientContext;
use rdkafka::config::{ClientConfig, RDKafkaLogLevel};
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::consumer::{Consumer, ConsumerContext};

struct CustomContext;
impl ClientContext for CustomContext {}
impl ConsumerContext for CustomContext {
    fn pre_rebalance(
        &self,
        _base_consumer: &rdkafka::consumer::BaseConsumer<Self>,
        rebalance: &rdkafka::consumer::Rebalance<'_>,
    ) {
        println!("Pre rebalance {:?}", rebalance);
    }

    fn post_rebalance(
        &self,
        _base_consumer: &rdkafka::consumer::BaseConsumer<Self>,
        rebalance: &rdkafka::consumer::Rebalance<'_>,
    ) {
        println!("Post rebalance {:?}", rebalance);
    }

    fn commit_callback(
        &self,
        result: rdkafka::error::KafkaResult<()>,
        _offsets: &rdkafka::TopicPartitionList,
    ) {
        println!("Committing orders: {:?}", result);
    }
}

type LoggingConsumer = StreamConsumer<CustomContext>;

#[tokio::main]
async fn main() {
    let context = CustomContext;
    let consumer: LoggingConsumer = ClientConfig::new()
        .set("group.id", "some_group")
        .set("bootstrap.servers", "localhost:9092")
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        .set_log_level(RDKafkaLogLevel::Debug)
        .create_with_context(context)
        .expect("consumer creation failed");

    consumer
        .subscribe(&["test"])
        .expect("can't subscribe to  specified topics");

    loop {}
}
