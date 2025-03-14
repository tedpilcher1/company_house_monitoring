use pulsar::{
    consumer::DeadLetterPolicy, producer, proto, Consumer, DeserializeMessage, Producer, Pulsar,
    SubType, TokioExecutor,
};

const PULSAR_ADDR: &str = "pulsar://localhost:6650";
const MAX_JOB_RETRY: usize = 3;

pub struct PulsarClient {
    pub internal_client: Pulsar<TokioExecutor>,
}

impl PulsarClient {
    pub async fn new() -> Self {
        Self {
            internal_client: Pulsar::builder(PULSAR_ADDR, TokioExecutor)
                .build()
                .await
                .expect("Should be able to create new pulsar client builder"),
        }
    }

    pub async fn create_producer(&self, topic: &str) -> Producer<TokioExecutor> {
        self.internal_client
            .producer()
            .with_topic(topic)
            .with_options(producer::ProducerOptions {
                schema: Some(proto::Schema {
                    r#type: proto::schema::Type::String as i32, // Or appropriate type for Job
                    ..Default::default()
                }),

                ..Default::default()
            })
            .build()
            .await
            .expect("Should be able to create producer")
    }

    pub async fn create_consumer<T>(
        &self,
        topic: &str,
        sub_type: SubType,
        subscription: &str,
    ) -> Consumer<T, TokioExecutor>
    where
        T: DeserializeMessage,
    {
        self.internal_client
            .consumer()
            .with_topic(topic)
            .with_subscription_type(sub_type) // exclusive for current testing
            .with_subscription(subscription)
            .with_dead_letter_policy(DeadLetterPolicy {
                max_redeliver_count: MAX_JOB_RETRY,
                dead_letter_topic: "DLQ".to_string(),
            })
            .build()
            .await
            .expect("Should be able to create consumer")
    }
}
