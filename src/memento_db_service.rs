use super::models::*;
use diesel::prelude::*;
use diesel::PgConnection;
use dotenv::dotenv;
use std::env;

pub fn write_memento(memento: &NewMemento) -> Memento {
    use super::schema::mementos;
    println!("DEBUG: Writing {:#?}", memento);
    let conn = get_db_connection();
    diesel::insert_into(mementos::table)
        .values(memento)
        .get_result::<Memento>(&conn)
        .expect("Error inserting new Memento")
}

fn get_db_connection() -> PgConnection {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("You must specify a URL for the database.");
    PgConnection::establish(&db_url).expect(&format!(
        "There was a problem establishing a connection to the database at {}.",
        db_url
    ))
}
