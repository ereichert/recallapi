use super::schema::mementos;
use serde::Deserialize;

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "mementos"]
pub struct NewMemento {
    pub prompt: String,
    pub details: String,
}

#[derive(Debug, Queryable)]
pub struct Memento {
    // TODO: This should probably be a GUID.
    pub id: i32,
    pub prompt: String,
    pub details: String,
}
