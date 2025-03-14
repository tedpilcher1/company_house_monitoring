use anyhow::Result;
use chrono::Utc;
use diesel::{
    delete, dsl::insert_into, query_dsl::methods::FilterDsl, Connection, ExpressionMethods,
    PgConnection, QueryResult, RunQueryDsl,
};
use uuid::Uuid;

use crate::database::{
    models::Subscription,
    schema::{notable_change, subscription},
};

use super::{
    models::{Company, NotableChange},
    schema::company,
};

pub struct DatabaseClient {
    conn: PgConnection,
}

impl DatabaseClient {
    pub fn new() -> Result<Self> {
        let database_url = std::env::var("DATABASE_URL")?;
        let conn = PgConnection::establish(&database_url)?;
        Ok(Self { conn })
    }

    pub fn create_subscription(
        &mut self,
        company_house_id: String,
        notable_changes: Vec<String>,
    ) -> Result<Uuid> {
        let id = Uuid::new_v4();
        let subscription = Subscription {
            id,
            company_house_id,
            created_at: Utc::now().naive_local(),
        };
        self.conn.transaction(|conn| {
            insert_into(subscription::table)
                .values(subscription)
                .execute(conn)?;

            for notable_change in notable_changes {
                insert_into(notable_change::table)
                    .values(NotableChange {
                        id: Uuid::new_v4(),
                        subscription_id: id,
                        field: notable_change,
                    })
                    .execute(conn)?;
            }
            QueryResult::Ok(())
        })?;

        Ok(id)
    }

    pub fn delete_subscription(&mut self, subscription_id: Uuid) -> Result<()> {
        self.conn.transaction(|conn| {
            delete(
                notable_change::table.filter(notable_change::subscription_id.eq(subscription_id)),
            )
            .execute(conn)?;
            delete(subscription::table.filter(subscription::id.eq(subscription_id)))
                .execute(conn)?;
            QueryResult::Ok(())
        })?;

        Ok(())
    }

    pub fn insert_company(&mut self, company_house_id: String) -> Result<()> {
        insert_into(company::table)
            .values(Company {
                company_house_id,
                first_monitored_at: Utc::now().naive_local(),
            })
            .execute(&mut self.conn)?;

        Ok(())
    }
}
