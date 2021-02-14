use super::models::*;
use diesel::{prelude::*, PgConnection};
use rocket_contrib::database;

pub trait RecallDb {
    fn get_all_mementos(&self) -> Mementos;
}

#[database("recall_db")]
pub struct RecallDbConn(PgConnection);

impl RecallDb for RecallDbConn {
    fn get_all_mementos(&self) -> Mementos {
        use super::schema::mementos::mementos::dsl::*;
        Mementos::new(mementos.load::<Memento>(&self.0).expect("Error loading posts"))
    }
}

pub fn write_memento(db_conn: &PgConnection, memento: &Memento) -> Memento {
    use super::schema::mementos::mementos;
    println!("DEBUG: Writing {:#?}", memento);
    diesel::insert_into(mementos::table)
        .values(memento)
        .get_result::<Memento>(db_conn)
        .expect("Error inserting new Memento")
}
