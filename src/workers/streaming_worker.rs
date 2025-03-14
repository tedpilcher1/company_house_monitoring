use pulsar::{Producer, TokioExecutor};

use crate::{company_house_sdk::client::CompanyHouseSDK, pulsar::client::PulsarClient};

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
}
