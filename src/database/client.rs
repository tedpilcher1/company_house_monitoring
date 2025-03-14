use anyhow::Result;
use chrono::Utc;
use diesel::{dsl::insert_into, Connection, PgConnection, RunQueryDsl};
use uuid::Uuid;

use crate::database::{models::Subscription, schema::subscription};

pub struct DatabaseClient {
    conn: PgConnection,
}

impl DatabaseClient {
    pub fn new() -> Result<Self> {
        let database_url = std::env::var("DATABASE_URL")?;
        let conn = PgConnection::establish(&database_url)?;
        Ok(Self { conn })
    }

    pub fn create_subscription(&mut self, company_house_id: String) -> Result<Uuid> {
        let id = Uuid::new_v4();
        let subscription = Subscription {
            id,
            company_house_id,
            created_at: Utc::now().naive_local(),
        };

        insert_into(subscription::table)
            .values(subscription)
            .execute(&mut self.conn)?;

        Ok(id)
    }
}
