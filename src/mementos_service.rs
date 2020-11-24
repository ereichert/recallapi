use rocket_contrib::json::Json;

use crate::{memento_db_service::RecallDb, models::Mementos};

pub fn get_mementos_as_json(db: impl RecallDb) -> Json<Mementos> {
    Json(db.get_all_mementos())
}

#[cfg(test)]
mod tests {
    use crate::models::Memento;
    use crate::{memento_db_service::RecallDb, models::Mementos};
    use rocket::{handler::Outcome, local::Client};
    use uuid::Uuid;

    #[test]
    fn get_all_mementos_returns_an_empty_result_when_no_mementos_have_been_created() {
        let recall_db_stub = new_recall_db_stub(vec![]);
        let expected_json_str = remove_whitespace_from_string(
            r#"{
            "mementos": [
            ]
        }"#,
        );

        let response = get_json_response(recall_db_stub);

        assert_eq!(response, expected_json_str);
    }

    #[test]
    fn get_all_mementos_returns_the_correct_json_representation() {
        let id0 = Uuid::new_v4();
        let id1 = Uuid::new_v4();
        let id2 = Uuid::new_v4();
        let recall_db_stub = new_recall_db_stub(vec![
            Memento {
                id: id0,
                prompt: "This is prompt 1.".to_owned(),
                details: "These are the details.".to_owned(),
            },
            Memento {
                id: id1,
                prompt: "This is prompt 2.".to_owned(),
                details: "These are the details.".to_owned(),
            },
            Memento {
                id: id2,
                prompt: "This is prompt 3.".to_owned(),
                details: "These are the details.".to_owned(),
            },
        ]);
        let expected_json_str = r#"{
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
        .replace("{id0}", &id0.hyphenated().to_string())
        .replace("{id1}", &id1.hyphenated().to_string())
        .replace("{id2}", &id2.hyphenated().to_string());
        let expected_json_str = remove_whitespace_from_string(&expected_json_str);

        let response = get_json_response(recall_db_stub);

        assert_eq!(response, expected_json_str);
    }

    fn new_recall_db_stub(test_mementos_vec: Vec<Memento>) -> RecallDbStub {
        let test_mementos = Mementos::new(test_mementos_vec);
        RecallDbStub { test_mementos }
    }

    fn remove_whitespace_from_string(json_string: &str) -> String {
        json_string.split_whitespace().collect()
    }

    fn get_json_response(recall_db_stub: RecallDbStub) -> String {
        let rocket = build_test_rocket();
        let client = Client::new(rocket)
            .expect("There was a problem building the rocket instance. This could be due to a bad config.");
        let req = client.get("/mementos");
        let rocket_responder = super::get_mementos_as_json(recall_db_stub);
        Outcome::from(req.inner(), rocket_responder)
            .unwrap()
            .body_string()
            .unwrap()
            .split_whitespace()
            .collect()
    }

    struct RecallDbStub {
        test_mementos: Mementos,
    }

    impl RecallDb for RecallDbStub {
        fn get_all_mementos(&self) -> Mementos {
            self.test_mementos.clone()
        }
    }

    fn build_test_rocket() -> rocket::Rocket {
        rocket::ignite()
    }
}
