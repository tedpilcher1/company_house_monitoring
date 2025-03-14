use anyhow::Result;
use bytes::Bytes;
use futures::StreamExt;
use pulsar::{Producer, TokioExecutor};

use crate::{
    company_house_sdk::{client::CompanyHouseSDK, types::CompanyStreamingResponse},
    pulsar::{client::PulsarClient, events::UpdateEvent},
};

pub const COMPANY_STREAMING_TOPIC: &str = "non-persistent://public/default/company-updates-topic";

pub struct StreamingWorker {
    update_event_producer: Producer<TokioExecutor>,
    company_house_sdk: CompanyHouseSDK,
}

impl StreamingWorker {
    pub async fn new() -> Self {
        let pulsar_client = PulsarClient::new().await;
        Self {
            update_event_producer: pulsar_client.create_producer(COMPANY_STREAMING_TOPIC).await,
            company_house_sdk: CompanyHouseSDK::new(),
        }
    }

    pub async fn do_work(&mut self, last_timepoint: Option<i32>) {
        let mut stream = self
            .company_house_sdk
            .connect_to_stream(last_timepoint)
            .await
            .expect("Should be able to connect to company update stream");

        let mut buffer: Vec<Vec<u8>> = Vec::new();
        while let Some(bytes_result) = stream.next().await {
            if let Ok(bytes) = bytes_result {
                match self.process_bytes(bytes, &mut buffer).await {
                    Ok(_) => println!("Successfully processed bytes"),
                    Err(e) => {
                        println!("Failed to process bytes, error: {:?}", e)
                    }
                }
            }
        }
    }

    async fn process_bytes(&mut self, bytes: Bytes, buffer: &mut Vec<Vec<u8>>) -> Result<()> {
        let chunks: Vec<&[u8]> = bytes.split_inclusive(|byte| byte == &b'\n').collect();
        for chunk in chunks {
            // skip heartbeat
            if chunk == &[10] {
                println!("Skipping heartbeat");
                continue;
            }

            let owned_chunk = chunk.to_owned();
            buffer.push(owned_chunk);
            if chunk.ends_with(&[10]) {
                self.process_chunk(buffer.concat()).await?;
                buffer.clear();
            }
        }
        Ok(())
    }

    async fn process_chunk(&mut self, chunk: Vec<u8>) -> Result<()> {
        let streaming_response: CompanyStreamingResponse = serde_json::from_slice(&chunk)?;
        let update_event = match (streaming_response.data, streaming_response.event) {
            (Some(data), Some(event)) => Some(UpdateEvent { event, data }),
            _ => None,
        };

        if let Some(update_event) = update_event {
            self.update_event_producer
                .send_non_blocking(update_event)
                .await?;
        }

        Ok(())
    }
}
