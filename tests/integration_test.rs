use chrono::{Duration, TimeZone, Utc};
use dotenv::dotenv;
use handler_lib::models::database::Database;
use handler_lib::supabase::supabase::SupabaseDb;

#[cfg(test)]
mod tests {
    use handler_lib::models::interfaces::Entry;

    use super::*;

    #[tokio::test]
    async fn database_test() {
        dotenv().ok();

        let database: SupabaseDb = Database::new();

        let from_date = Utc.ymd(2100, 1, 1).and_hms(1, 1, 1);
        let past_date = from_date + Duration::days(-1);
        let future_date = from_date + Duration::days(1);

        let from_entry = Entry {
            date: from_date.format("%Y-%m-%d").to_string(),
            person: "test".to_string(),
        };
        let past_entry = Entry {
            date: past_date.format("%Y-%m-%d").to_string(),
            person: "test".to_string(),
        };
        let future_entry = Entry {
            date: future_date.format("%Y-%m-%d").to_string(),
            person: "test".to_string(),
        };

        _ = database.add(from_entry.clone()).await;
        _ = database.add(past_entry.clone()).await;
        _ = database.add(future_entry.clone()).await;

        let response = database.get_entries(from_date, future_date).await.unwrap();

        let parsed_response: Vec<Entry> = serde_json::from_str(response.as_str()).unwrap();

        _ = database.delete(from_entry.clone().date).await;
        _ = database.delete(past_entry.clone().date).await;
        _ = database.delete(future_entry.clone().date).await;

        assert_eq!(parsed_response.contains(&from_entry), true);
        assert_eq!(parsed_response.contains(&past_entry), false);
        assert_eq!(parsed_response.contains(&future_entry), true);
    }
}
