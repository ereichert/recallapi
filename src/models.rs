use super::schema::memories;
use serde::Deserialize;

#[derive(Debug, Insertable)]
#[table_name = "memories"]
pub struct RecallMemory<'a> {
    pub prompt: &'a str,
    pub details: &'a str,
}

// TODO: I don't know that I need this in the long run but I need it for deserializing JSON from a text file.
#[derive(Debug, Deserialize)]
pub struct OwnedRecallMemory {
    pub prompt: String,
    pub details: String,
}

impl<'a> RecallMemory<'a> {
    pub fn new(prompt: &'a str, details: &'a str) -> RecallMemory<'a> {
        RecallMemory { prompt, details }
    }
}

#[derive(Debug, Queryable)]
pub struct Memory {
    pub id: i32,
    pub prompt: String,
    pub details: String,
}
