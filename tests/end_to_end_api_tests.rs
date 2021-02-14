use diesel::prelude::*;
use diesel::{Connection, PgConnection};
use dotenv::dotenv;
use parking_lot::Mutex;
use recallapi::api::build_rocket;
use recallapi::db_services;
use recallapi::models::Memento;
use rocket::{http::Status, local::Client};
use std::env;

#[test]
fn get_all_mementos_returns_an_empty_result_when_no_mementos_have_been_created() {
    let db_conn = get_db_connection();
    run_test_with_clean_db(&db_conn, || {
        let expected_json_str = r#"{
            "mementos": [
            ]
        }"#;
        let expected_json_str = remove_whitespace_from_string(&expected_json_str);

        let rocket = build_rocket();
        let client = Client::new(rocket)
            .expect("There was a problem building the rocket instance. This could be due to a bad config.");

        let response = client.get("/mementos").dispatch().body_string().unwrap();
        let received_json_str = remove_whitespace_from_string(&response);

        assert_eq!(received_json_str, expected_json_str);
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

        let expected_json_str: String = r#"{
            "mementos": [
                {
                    "id":"{id0}",
                    "prompt": "This is prompt 1.",
                    "details": "These are the details."
                },
                {
                    "id":"{id1}",
                    "prompt": "This is prompt 2.",
                    "details": "These are the details."
                },
                {
                    "id":"{id2}",
                    "prompt": "This is prompt 3.",
                    "details": "These are the details."
                }
            ]
        }"#
        .replace("{id0}", &test_mementos[0].id.hyphenated().to_string())
        .replace("{id1}", &test_mementos[1].id.hyphenated().to_string())
        .replace("{id2}", &test_mementos[2].id.hyphenated().to_string());
        let expected_json_str = remove_whitespace_from_string(&expected_json_str);

        let rocket = build_rocket();
        let client = Client::new(rocket)
            .expect("There was a problem building the rocket instance. This could be due to a bad config.");

        let response = client.get("/mementos").dispatch().body_string().unwrap();
        let received_json_str = remove_whitespace_from_string(&response);

        assert_eq!(received_json_str, expected_json_str);
    });
}

#[test]
fn get_all_mementos_returns_200() {
    let rocket = build_rocket();
    let client = Client::new(rocket).expect("Invalid Rocket config detected.");

    let response = client.get("/mementos").dispatch();

    assert_eq!(response.status(), Status::Ok)
}

#[test]
fn post_new_memento_returns_200() {
    let rocket = build_rocket();
    let client = Client::new(rocket).expect("Invalid Rocket config detected.");

    let response = client.post("/mementos").dispatch();

    assert_eq!(response.status(), Status::Ok)
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

fn remove_whitespace_from_string(json_string: &str) -> String {
    json_string.split_whitespace().collect()
}
