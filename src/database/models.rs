use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::database::schema::company)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Company {
    pub company_house_id: String,
    pub first_monitored_at: NaiveDateTime,
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::database::schema::subscription)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Subscription {
    pub id: Uuid,
    pub company_house_id: String,
    pub created_at: NaiveDateTime,
    pub url: String,
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::database::schema::notable_change)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NotableChange {
    pub id: Uuid,
    pub subscription_id: Uuid,
    pub field: String,
}

#[derive(Queryable, Selectable, Insertable, Deserialize, Serialize, Clone)]
#[diesel(table_name = crate::database::schema::company_snapshot)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CompanySnapshot {
    pub id: Uuid,
    pub company_house_id: String,
    pub snapshot_data: Value,
    pub recieved_at: NaiveDateTime,
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::database::schema::processed_update)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ProcessedUpdate {
    pub timepoint: i32,
    pub processed_at: NaiveDateTime,
}
