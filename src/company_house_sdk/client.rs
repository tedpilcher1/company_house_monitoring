use std::{collections::HashMap, env};

use anyhow::Result;
use bytes::Bytes;
use futures::Stream;
use lazy_static::lazy_static;
use reqwest::{header, Client};

use super::types::CompanyData;

const COMPANY_STREAMING_URL: &str = "https://stream.companieshouse.gov.uk/companies";

lazy_static! {
    static ref API_KEY: String =
        env::var("COMPANY_HOUSE_STREAMING_API_KEY").expect("Streaming API KEY should be set");
}

pub struct CompanyHouseSDK {
    client: Client,
}

impl CompanyHouseSDK {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn get_company(&self, company_house_number: String) -> Result<CompanyData> {
        let url = format!(
            "https://api.company-information.service.gov.uk/company/{}",
            company_house_number
        );
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "Authorization",
            header::HeaderValue::from_str(&format!("{}", API_KEY.as_str()))?,
        );
        let response = self.client.get(url).headers(headers).send().await?;
        let company_data: CompanyData = response.json().await?;
        Ok(company_data)
    }

    pub async fn connect_to_stream(
        &self,
        timepoint: Option<i32>,
    ) -> Result<impl Stream<Item = Result<Bytes, reqwest::Error>>> {
        let mut params = HashMap::new();
        if let Some(timepoint) = timepoint {
            params.insert("timepoint", timepoint + 1);
        }

        let mut headers = header::HeaderMap::new();
        headers.insert(
            "Authorization",
            header::HeaderValue::from_str(&format!("{}", API_KEY.as_str()))?,
        );

        Ok(self
            .client
            .get(COMPANY_STREAMING_URL)
            .headers(headers)
            .query(&params)
            .send()
            .await?
            .bytes_stream())
    }
}
