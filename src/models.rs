use super::schema::mementos::mementos;
use serde::{Deserialize, Serialize};
use uuid;
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Insertable, Serialize, Queryable)]
#[table_name = "mementos"]
pub struct Memento {
    #[serde(skip_deserializing)]
    pub id: uuid::Uuid,
    pub prompt: String,
    pub details: String,
}

impl Memento {
    pub fn new(prompt: String, details: String) -> Memento {
        Memento {
            id: Uuid::new_v4(),
            prompt: prompt,
            details: details,
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Mementos {
    mementos: Vec<Memento>,
}

impl Mementos {
    pub fn new(mementos: Vec<Memento>) -> Mementos {
        Mementos { mementos }
    }
}
