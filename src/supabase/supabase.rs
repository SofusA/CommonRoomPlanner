use async_trait::async_trait;
use chrono::DateTime;
use chrono::Duration;
use chrono::Utc;
use reqwest::StatusCode;
use postgrest::Postgrest;

use crate::models::constants::*;
use crate::models::database::*;
use crate::models::interfaces::*;

#[derive(Clone)]
pub struct SupabaseDb {
}

#[async_trait]
impl Database for SupabaseDb {
    async fn delete(&self, entry_id: DateFormat) -> Result<StatusCode, String> {
        let client = match Self::get_client() {
            Ok(res) => res,
            Err(err) => return Err(err),
        };

        let database_table_name = match database_table_name() {
            Ok(res) => res,
            Err(err) => return Err(err.to_string()),
        };

        let resp = match client
            .from(database_table_name)
            .delete()
            .eq("date", entry_id)
            .execute()
            .await
        {
            Ok(res) => res,
            Err(err) => return Err(format!("Error from Supabase: {}", err)),
        };

        return Ok(resp.status());
    }

    async fn add(&self, entry: Entry) -> Result<StatusCode, String> {
        let client = match Self::get_client() {
            Ok(res) => res,
            Err(err) => return Err(err),
        };

        let serialised_entry = match serde_json::to_string(&entry) {
            Ok(res) => res,
            Err(err) => return Err(err.to_string()),
        };

        let database_table_name = match database_table_name() {
            Ok(res) => res,
            Err(err) => return Err(err.to_string()),
        };

        let resp = match client
            .from(database_table_name)
            .insert(format!("[{}]", serialised_entry))
            .execute()
            .await
        {
            Ok(res) => res,
            Err(err) => return Err(format!("Error from Supabase: {}", err)),
        };

        return Ok(resp.status());

    }

    async fn get_next_weeks(&self, from: DateTime<Utc>, to: WeekRequest) -> Result<String, String> {
        let client = match Self::get_client() {
            Ok(res) => res,
            Err(err) => return Err(err),
        };

        let database_table_name = match database_table_name() {
            Ok(res) => res,
            Err(err) => return Err(err.to_string()),
        };

        let next_week = from + Duration::weeks(to.weeks);

        let resp = match client
            .from(database_table_name)
            .select("date, person")
            .gte("date", from.format("%Y-%m-%d").to_string())
            .lt("date", next_week.format("%Y-%m-%d").to_string())
            .order("date.asc")
            .execute()
            .await
        {
            Ok(res) => res,
            Err(err) => return Err(format!("Error from Supabase: {}", err)),
        };

        match resp.json::<Vec<Entry>>().await {
            Ok(res) => {
                return Ok(match serde_json::to_string(&res) {
                    Ok(res) => res,
                    Err(_) => "error serialising json".to_string(),
                })
            }
            Err(err) => return Err(format!("Error getting latest booking: {}", err)),
        }
    }

    fn new() -> SupabaseDb {
        return SupabaseDb {};
    }

    fn get_client() -> Result<Postgrest, String> {
        let url = match database_url() {
            Ok(res) => res,
            Err(err) => return Err(err),
        };

        let database_secret = match database_secret() {
            Ok(res) => res,
            Err(err) => return Err(err.to_string()),
        };

        return Ok(Postgrest::new(url).insert_header("apikey", database_secret));
    }
}