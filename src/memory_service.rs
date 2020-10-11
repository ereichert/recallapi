use super::models::*;
use diesel::prelude::*;
use diesel::PgConnection;
use dotenv::dotenv;
use std::env;

pub fn write_memory(memory: &RecallMemory) -> Memory {
    use super::schema::memories;
    println!("DEBUG: Writing {:#?}", memory);
    let conn = get_db_connection();
    diesel::insert_into(memories::table)
        .values(memory)
        .get_result::<Memory>(&conn)
        .expect("Error inserting new Memory")
}

fn get_db_connection() -> PgConnection {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("You must specify a URL for the database.");
    PgConnection::establish(&db_url).expect(&format!(
        "There was a problem establishing a connection to the database at {}.",
        db_url
    ))
}
