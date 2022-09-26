use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Entry {
    pub date: DateFormat,
    pub person: String
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EntryId {
    pub date: String,
}


pub type DateFormat = String;