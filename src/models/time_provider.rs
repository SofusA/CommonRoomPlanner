use chrono::{DateTime, Utc};

pub trait TimeProvider {
    fn get_current_time() -> DateTime<Utc>;
    fn new() -> Self;
}
