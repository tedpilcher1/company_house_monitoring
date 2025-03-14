use crate::{
    database::client::DatabaseClient,
    pulsar::{client::PulsarClient, events::UpdateEvent},
};
use pulsar::{Consumer, SubType, TokioExecutor};

use super::streaming_worker::COMPANY_STREAMING_TOPIC;

const SUB: &str = "monitored-update-sub";
const SUB_TYPE: SubType = SubType::Shared;

pub struct ProcessUpdatesWorker {
    database: DatabaseClient,
    consumer: Consumer<UpdateEvent, TokioExecutor>,
}

impl ProcessUpdatesWorker {
    pub async fn new() -> Self {
        let pulsar_client = PulsarClient::new().await;

        Self {
            database: DatabaseClient::new().expect("Should be able to connect to db"),
            consumer: pulsar_client
                .create_consumer(COMPANY_STREAMING_TOPIC, SUB_TYPE, SUB)
                .await,
        }
    }
}
