use std::collections::HashSet;

use anyhow::{bail, Result};
use futures::TryStreamExt;
use pulsar::{consumer::Message, Consumer, SubType, TokioExecutor};
use reqwest::Client;

use crate::{
    database::{client::DatabaseClient, models::NotableChange},
    pulsar::{client::PulsarClient, events::NotificationEvent},
};

use super::{diff_json, process_updates_worker::NOTIFICATION_TOPIC};
const SUB: &str = "notification-event-sub";
const SUB_TYPE: SubType = SubType::Shared;

pub struct NotificationWorker {
    database: DatabaseClient,
    consumer: Consumer<NotificationEvent, TokioExecutor>,
    client: Client,
}

impl NotificationWorker {
    pub async fn new() -> Self {
        let pulsar_client = PulsarClient::new().await;
        Self {
            database: DatabaseClient::new().expect("Should be able to connect to db"),
            consumer: pulsar_client
                .create_consumer(NOTIFICATION_TOPIC, SUB_TYPE, SUB)
                .await,
            client: Client::new(),
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
    async fn process_message(&mut self, msg: &Message<NotificationEvent>) -> Result<()> {
        let notification_event = msg.deserialize()?;
        let subscriptions = self
            .database
            .get_company_subscriptions(&notification_event.company_house_id)?;

        let fields_diff = self.get_snapshot_diff(&notification_event.company_house_id)?;

        for subscription in &subscriptions {
            if let Ok(notable_changes) = self.database.get_notable_changes(&subscription.id) {
                if notable_changes.is_empty()
                    || self.contains_notable_change(notable_changes, &fields_diff)
                {
                    let _ = self
                        .client
                        .post(&subscription.url)
                        .json(&notification_event.company_house_id)
                        .send()
                        .await;
                }
            }
        }

        Ok(())
    }

    fn get_snapshot_diff(&mut self, company_house_id: &String) -> Result<HashSet<String>> {
        let last_two_snapshots = self
            .database
            .get_last_two_company_snapshots(company_house_id)?;

        let snapshot_1 = match last_two_snapshots.0.snapshot_data.as_object() {
            Some(obj) => obj,
            None => bail!("First snapshot data is not a valid JSON object"),
        };
        let snapshot_2 = match last_two_snapshots.1.snapshot_data.as_object() {
            Some(obj) => obj,
            None => bail!("Second snapshot data is not a valid JSON object"),
        };
        diff_json(snapshot_1, snapshot_2)
    }

    fn contains_notable_change(
        &self,
        notable_changes: Vec<NotableChange>,
        fields_diff: &HashSet<String>,
    ) -> bool {
        for notable_change in notable_changes {
            if fields_diff.contains(&notable_change.field) {
                return true;
            }
        }
        false
    }
}
