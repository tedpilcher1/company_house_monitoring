use pulsar::{producer, proto, Producer, Pulsar, TokioExecutor};

const PULSAR_ADDR: &str = "pulsar://localhost:6650";

pub struct PulsarClient {
    internal_client: Pulsar<TokioExecutor>,
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
}
