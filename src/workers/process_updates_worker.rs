use crate::{
    database::client::DatabaseClient,
    pulsar::{client::PulsarClient, events::UpdateEvent},
};
use anyhow::Result;
use futures::TryStreamExt;
use pulsar::{consumer::Message, Consumer, SubType, TokioExecutor};

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

    pub async fn do_work(&mut self) {
        while let Some(msg) = self
            .consumer
            .try_next()
            .await
            .expect("Should be able to wait for new message.")
        {
            match self.process_message(&msg) {
                Ok(_) => {
                    self.consumer
                        .ack(&msg)
                        .await
                        .expect("Should be able to ack msg");
                }
                Err(_) => self
                    .consumer
                    .nack(&msg)
                    .await
                    .expect("Should be able to nack msg"),
            }
        }
    }

    fn process_message(&mut self, msg: &Message<UpdateEvent>) -> Result<()> {
        let update_event = msg.deserialize()?;
        let update = update_event.data;
        let company_house_id = update.company_number.clone();
        self.database
            .insert_company_snapshot(&company_house_id, serde_json::to_value(update)?)?;
        self.database
            .insert_processed_update(update_event.event.timepoint)?;
        Ok(())
    }
}
