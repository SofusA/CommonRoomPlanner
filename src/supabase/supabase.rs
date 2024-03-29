use async_trait::async_trait;
use chrono::DateTime;
use chrono::Utc;
use postgrest::Postgrest;
use reqwest::StatusCode;

use crate::models::constants::*;
use crate::models::database::*;
use crate::models::interfaces::*;

#[derive(Clone)]
pub struct SupabaseDb {}

#[async_trait]
impl Database for SupabaseDb {
    async fn delete(&self, entry_id: DateFormat) -> Result<StatusCode, String> {
        let client = match Self::get_client() {
            Ok(res) => res,
            Err(err) => return Err(err),
        };

        let resp = match client
            .from("Bookings")
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

        let resp = match client
            .from("Bookings")
            .insert(format!("[{}]", serialised_entry))
            .execute()
            .await
        {
            Ok(res) => res,
            Err(err) => return Err(format!("Error from Supabase: {}", err)),
        };

        return Ok(resp.status());
    }

    async fn get_entries(&self, from: DateTime<Utc>, to: DateTime<Utc>) -> Result<String, String> {
        let client = match Self::get_client() {
            Ok(res) => res,
            Err(err) => return Err(err),
        };

        let resp = match client
            .from("Bookings")
            .select("date, person")
            .gte("date", from.format("%Y-%m-%d").to_string())
            .lte("date", to.format("%Y-%m-%d").to_string())
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
