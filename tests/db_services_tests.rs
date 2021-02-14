use db_services::{RecallDb, RecallDbConn};
use diesel::prelude::*;
use diesel::{Connection, PgConnection};
use dotenv::dotenv;
use parking_lot::Mutex;
use recallapi::api::build_rocket;
use recallapi::db_services;
use recallapi::models::Memento;
use std::env;

#[test]
fn get_all_mementos_returns_an_empty_result_when_no_mementos_have_been_created() {
    let db_conn = get_db_connection();
    run_test_with_clean_db(&db_conn, || {
        let rocket = build_rocket();
        let recall_db = RecallDbConn::get_one(&rocket).expect("Failed to get instance of RecallDBConn");
        let retrieved_mementos = recall_db.get_all_mementos();
        assert_eq!(retrieved_mementos.len(), 0);
    });
}

#[test]
fn get_all_mementos_returns_the_correct_json_representation() {
    let db_conn = get_db_connection();
    run_test_with_clean_db(&db_conn, || {
        let memento1 = Memento::new("This is prompt 1.".to_owned(), "These are the details.".to_owned());
        let memento2 = Memento::new("This is prompt 2.".to_owned(), "These are the details.".to_owned());
        let memento3 = Memento::new("This is prompt 3.".to_owned(), "These are the details.".to_owned());
        let test_mementos = vec![memento1, memento2, memento3];

        for memento in &test_mementos {
            db_services::write_memento(&db_conn, &memento);
        }

        let rocket = build_rocket();
        let recall_db = RecallDbConn::get_one(&rocket).expect("Failed to get instance of RecallDBConn");
        let retrieved_mementos = recall_db.get_all_mementos();

        assert_eq!(retrieved_mementos.len(), 3);
    });
}

fn get_db_connection() -> PgConnection {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("You must specify a URL for the database.");
    PgConnection::establish(&db_url).expect(&format!(
        "There was a problem establishing a connection to the database at {}.",
        db_url
    ))
}

static DB_LOCK: Mutex<()> = parking_lot::const_mutex(());
pub fn run_test_with_clean_db<T>(db_conn: &PgConnection, test: T) -> ()
where
    T: FnOnce() -> (),
{
    let _lock = DB_LOCK.lock();
    diesel::sql_query("TRUNCATE mementos.mementos RESTART IDENTITY;")
        .execute(db_conn)
        .expect("Could not truncate the mementos table.");

    test();
}
