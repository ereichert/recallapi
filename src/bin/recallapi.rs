#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[get("/world")]
fn world() -> &'static str {
    "hello world!"
}

fn build_rocket() -> rocket::Rocket {
    rocket::ignite().mount("/hello", routes![world])
}

fn main() {
    println!("Starting up Recall API 0.0.1.");
    build_rocket().launch();
}

#[cfg(test)]
mod tests {
    use crate::build_rocket;
    use rocket::{local::Client, http::Status};

    #[test]
    fn get_hello_world_returns_200() {
        let rocket = build_rocket();
        let client = Client::new(rocket).expect("invalid Rocket config detected");

        let response = client.get("/hello/world").dispatch();

        assert_eq!(response.status(), Status::Ok)
    }

    #[test]
    fn this_is_the_second_test() {
        unimplemented!();
    }
}
