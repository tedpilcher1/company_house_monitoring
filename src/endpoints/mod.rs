use std::sync::{Arc, Mutex};

use serde::Deserialize;

use crate::database::client::DatabaseClient;

pub mod api_endpoints;

#[derive(Clone)]
pub struct AppState {
    pub database: Arc<Mutex<DatabaseClient>>,
}

#[derive(Deserialize)]
pub struct SubscribeRequest {
    notable_changes: Vec<String>,
    url: String,
}
