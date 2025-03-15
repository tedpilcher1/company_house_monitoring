use pulsar::{producer, DeserializeMessage, Error as PulsarError, SerializeMessage};
use serde::{Deserialize, Serialize};

use crate::company_house_sdk::types::{CompanyData, Event};

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateEvent {
    pub event: Event,
    pub data: CompanyData,
}

impl SerializeMessage for UpdateEvent {
    fn serialize_message(input: Self) -> Result<producer::Message, PulsarError> {
        let payload = serde_json::to_vec(&input).map_err(|e| PulsarError::Custom(e.to_string()))?;
        Ok(producer::Message {
            payload,
            ..Default::default()
        })
    }
}

impl DeserializeMessage for UpdateEvent {
    type Output = Result<UpdateEvent, serde_json::Error>;

    fn deserialize_message(payload: &pulsar::Payload) -> Self::Output {
        serde_json::from_slice(&payload.data)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NotificationEvent {
    pub company_house_id: String,
}

impl NotificationEvent {
    pub fn new(company_house_id: String) -> Self {
        Self { company_house_id }
    }
}

impl SerializeMessage for NotificationEvent {
    fn serialize_message(input: Self) -> Result<producer::Message, PulsarError> {
        let payload = serde_json::to_vec(&input).map_err(|e| PulsarError::Custom(e.to_string()))?;
        Ok(producer::Message {
            payload,
            ..Default::default()
        })
    }
}

impl DeserializeMessage for NotificationEvent {
    type Output = Result<NotificationEvent, serde_json::Error>;

    fn deserialize_message(payload: &pulsar::Payload) -> Self::Output {
        serde_json::from_slice(&payload.data)
    }
}
