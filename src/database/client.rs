use anyhow::Result;
use chrono::{NaiveDateTime, Utc};
use diesel::{delete, insert_into, prelude::*};
use serde_json::Value;
use uuid::Uuid;

use crate::database::{
    models::Subscription,
    schema::{notable_change, subscription},
};

use super::{
    models::{Company, CompanySnapshot, NotableChange, ProcessedUpdate},
    schema::{company, company_snapshot, processed_update},
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
        url: String,
    ) -> Result<Uuid> {
        let id = Uuid::new_v4();
        let subscription = Subscription {
            id,
            company_house_id: company_house_id.clone(),
            created_at: Utc::now().naive_local(),
            url,
        };
        self.conn.transaction(|conn| {
            insert_into(company::table)
                .values(Company {
                    company_house_id,
                    first_monitored_at: Utc::now().naive_local(),
                })
                .on_conflict_do_nothing()
                .execute(conn)?;

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
            let company_house_id = subscription::table
                .filter(subscription::id.eq(subscription_id))
                .select(subscription::company_house_id)
                .for_update()
                .get_result::<String>(conn)?;

            delete(
                notable_change::table.filter(notable_change::subscription_id.eq(subscription_id)),
            )
            .execute(conn)?;
            delete(subscription::table.filter(subscription::id.eq(subscription_id)))
                .execute(conn)?;

            let count = subscription::table
                .filter(subscription::company_house_id.eq(&company_house_id))
                .count()
                .execute(conn)?;

            if count == 0 {
                delete(company::table.filter(company::company_house_id.eq(company_house_id)))
                    .execute(conn)?;
            }

            QueryResult::Ok(())
        })?;

        Ok(())
    }

    pub fn insert_processed_update(&mut self, timepoint: i32) -> Result<()> {
        insert_into(processed_update::table)
            .values(ProcessedUpdate {
                timepoint,
                processed_at: Utc::now().naive_local(),
            })
            .execute(&mut self.conn)?;
        Ok(())
    }

    pub fn get_last_processed_timepoint(&mut self) -> Result<Option<i32>> {
        Ok(processed_update::table
            .order_by(processed_update::timepoint.desc())
            .select(processed_update::timepoint)
            .first::<i32>(&mut self.conn)
            .optional()?)
    }

    pub fn insert_company_snapshot(
        &mut self,
        company_house_id: &String,
        snapshot_data: Value,
    ) -> Result<Option<String>> {
        let company = self.conn.transaction(|conn| {
            let company = company::table
                .filter(company::company_house_id.eq(company_house_id))
                .select(company::all_columns)
                .get_result::<Company>(conn)
                .optional()?;

            if company.is_some() {
                insert_into(company_snapshot::table)
                    .values(CompanySnapshot {
                        id: Uuid::new_v4(),
                        company_house_id: company_house_id.to_string(),
                        snapshot_data,
                        recieved_at: Utc::now().naive_local(),
                    })
                    .execute(conn)?;
            }

            QueryResult::Ok(company)
        })?;

        match company {
            Some(company) => Ok(Some(company.company_house_id)),
            None => Ok(None),
        }
    }

    pub fn get_company_snapshots(
        &mut self,
        company_house_id: String,
        from_date: Option<NaiveDateTime>,
        to_date: Option<NaiveDateTime>,
    ) -> Result<Vec<CompanySnapshot>> {
        let snapshots = company_snapshot::table
            .filter(
                company_snapshot::company_house_id.eq(company_house_id).and(
                    company_snapshot::recieved_at
                        .le(to_date.unwrap_or(Utc::now().naive_local()))
                        .and(company_snapshot::recieved_at.ge(from_date.unwrap_or_default())),
                ),
            )
            .select(company_snapshot::all_columns)
            .get_results::<CompanySnapshot>(&mut self.conn)?;

        Ok(snapshots)
    }
}
