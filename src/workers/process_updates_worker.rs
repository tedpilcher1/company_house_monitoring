use crate::{
    database::client::DatabaseClient,
    pulsar::{
        client::PulsarClient,
        events::{NotificationEvent, UpdateEvent},
    },
};
use anyhow::Result;
use futures::TryStreamExt;
use pulsar::{consumer::Message, Consumer, Producer, SubType, TokioExecutor};

use super::streaming_worker::COMPANY_STREAMING_TOPIC;

pub const NOTIFICATION_TOPIC: &str = "non-persistent://public/default/notification-topic";
const SUB: &str = "monitored-update-sub";
const SUB_TYPE: SubType = SubType::Shared;

pub struct ProcessUpdatesWorker {
    database: DatabaseClient,
    consumer: Consumer<UpdateEvent, TokioExecutor>,
    notification_producer: Producer<TokioExecutor>,
}

impl ProcessUpdatesWorker {
    pub async fn new() -> Self {
        let pulsar_client = PulsarClient::new().await;

        Self {
            database: DatabaseClient::new().expect("Should be able to connect to db"),
            consumer: pulsar_client
                .create_consumer(COMPANY_STREAMING_TOPIC, SUB_TYPE, SUB)
                .await,
            notification_producer: pulsar_client.create_producer(NOTIFICATION_TOPIC).await,
        }
    }

    pub async fn do_work(&mut self) {
        while let Some(msg) = self
            .consumer
            .try_next()
            .await
            .expect("Should be able to wait for new message.")
        {
            match self.process_message(&msg).await {
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

    async fn process_message(&mut self, msg: &Message<UpdateEvent>) -> Result<()> {
        let update_event = msg.deserialize()?;
        let update = update_event.data;
        let company_house_id = update.company_number.clone();

        if let Some(company_house_id) = self
            .database
            .insert_company_snapshot(&company_house_id, serde_json::to_value(update)?)?
        {
            let _ = self
                .notification_producer
                .send_non_blocking(NotificationEvent::new(company_house_id))
                .await;
        }
        self.database
            .insert_processed_update(update_event.event.timepoint)?;
        Ok(())
    }
}
