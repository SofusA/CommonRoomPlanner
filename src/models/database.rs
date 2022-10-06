use async_trait::async_trait;
use chrono::{Utc, DateTime};
use postgrest::Postgrest;
use reqwest::StatusCode;
use crate::models::interfaces::{Entry, DateFormat};

#[async_trait]
pub trait Database {
    async fn delete(&self, entry_id: DateFormat) -> Result<StatusCode, String>;
    async fn add(&self, entry: Entry) -> Result<StatusCode, String>;
    async fn get_entries(&self, from: DateTime<Utc>, to: DateTime<Utc>) -> Result<String, String>;
    fn get_client() -> Result<Postgrest, String>;

    fn new() -> Self;
}