#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[post("/mementos")]
fn add_memento() -> &'static str {
    "memento received!"
}

fn build_rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![add_memento])
}

fn main() {
    println!("Starting up Recall API 0.0.1.");
    build_rocket().launch();
}

#[cfg(test)]
mod tests {
    use crate::build_rocket;
    use rocket::{http::Status, local::Client};

    #[test]
    fn post_new_memento_returns_200() {
        let rocket = build_rocket();
        let client = Client::new(rocket).expect("Invalid Rocket config detected.");

        let response = client.post("/mementos").dispatch();

        assert_eq!(response.status(), Status::Ok)
    }
}
