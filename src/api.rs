use crate::models::Mementos;
use crate::{memento_db_service::RecallDbConn, mementos_service};
use rocket::{get, post, routes};
use rocket_contrib::json::Json;

#[post("/mementos")]
fn add_memento(_db_conn: RecallDbConn) -> &'static str {
    "memento received!"
}

#[get("/mementos")]
fn get_all_mementos(db_conn: RecallDbConn) -> Json<Mementos> {
    mementos_service::get_mementos_as_json(db_conn)
}

pub fn build_rocket() -> rocket::Rocket {
    println!("Starting up Recall API 0.0.1.");
    rocket::ignite()
        .attach(RecallDbConn::fairing())
        .mount("/", routes![add_memento, get_all_mementos])
}
