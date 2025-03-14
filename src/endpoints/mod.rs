use std::sync::{Arc, Mutex};

use chrono::NaiveDateTime;
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

#[derive(Deserialize)]
struct DateRange {
    from_date: Option<NaiveDateTime>,
    to_date: Option<NaiveDateTime>,
}
