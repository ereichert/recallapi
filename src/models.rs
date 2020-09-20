use super::schema::memories;

#[derive(Debug)]
#[derive(Insertable)]
#[table_name="memories"]
pub struct NewMemory<'a> {
    pub prompt: &'a str,
    pub details: &'a str,
}

impl<'a> NewMemory<'a> {
    pub fn new(prompt: &'a str, details: &'a str) -> NewMemory<'a> {
        NewMemory {
            prompt,
            details,
        }
    }
}

#[derive(Debug)]
#[derive(Queryable)]
pub struct Memory {
    pub id: i32,
    pub prompt: String,
    pub details: String,
}