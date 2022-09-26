use async_trait::async_trait;
use postgrest::Postgrest;
use reqwest::StatusCode;
use crate::models::entry::{Entry, EntryId};

#[async_trait]
pub trait Database {
    async fn delete(&self, entry_id: EntryId) -> Result<String, String>;
    async fn add(&self, entry: Entry) -> Result<StatusCode, String>;
    async fn get_latest(&self) -> Result<String, String>;
    fn get_client() -> Result<Postgrest, String>;

    fn new() -> Self;
}

#[derive(Clone)]
pub struct SupabaseDb {
}
