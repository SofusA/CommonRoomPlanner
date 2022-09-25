use async_trait::async_trait;
use postgrest::Postgrest;
use crate::models::entry::{DateFormat, Entry};

#[async_trait]
pub trait Database {
    async fn delete(&self, date: DateFormat) -> Result<String, String>;
    async fn add(&self, entry: Entry) -> Result<String, String>;
    async fn get_latest(&self) -> Result<String, String>;
    fn get_client() -> Postgrest;

    fn new() -> Self;
}

#[derive(Clone)]
pub struct SupabaseDb {
}
