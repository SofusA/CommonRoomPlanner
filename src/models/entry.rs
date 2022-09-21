use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Entry {
    pub date: DateFormat,
    pub person: String
}

pub type DateFormat = String;