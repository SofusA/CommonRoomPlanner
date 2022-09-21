use super::entry::{DateFormat, Entry};

trait Database {
    fn delete(&self, date: DateFormat) -> Result<(), &'static str>;
    fn add(&self, entry: Entry) ->  Result<(), &'static str>;
    fn get(&self, date: DateFormat) -> Result<(), &'static str>;
}