use recall_api::{memento_db_service, models::NewMemento};
use std::fmt;
use std::io::BufReader;
use std::io::{self, Read};
use std::{error, fs::File};

fn main() {
    const TEST_MEMENTO_JSON_FILE: &'static str = "./test-data/mem_file.json";
    let write_result = read_memento_from_file(TEST_MEMENTO_JSON_FILE).map(|memento| {
        println!("DEBUG: Sending Memento to MementoService {:#?}", memento);
        memento_db_service::write_memento(&memento)
    });

    match write_result {
        Ok(memento) => {
            println!("Successfuly saved new Memento.");
            println!();
            println!("Prompt: \n\n{}", memento.prompt);
            println!();
            println!("Details: \n\n{}", memento.details);
        }
        Err(err) => print!("Failed to write Memento because of error: {:#?}", err),
    }
}

#[derive(Debug)]
enum MementoReadErrors {
    IO(io::Error),
    Deserilization(serde_json::Error),
}

impl fmt::Display for MementoReadErrors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MementoReadErrors::IO(ref err) => write!(f, "MementoReadErrors::IO error: {}", err),
            MementoReadErrors::Deserilization(ref err) => write!(f, "MementoReadErrors::Deserilization error: {}", err),
        }
    }
}

impl error::Error for MementoReadErrors {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            MementoReadErrors::IO(ref err) => Some(err),
            MementoReadErrors::Deserilization(ref err) => Some(err),
        }
    }
}

impl From<io::Error> for MementoReadErrors {
    fn from(err: io::Error) -> Self {
        MementoReadErrors::IO(err)
    }
}

fn read_memento_from_file(file_location: &str) -> Result<NewMemento, MementoReadErrors> {
    match File::open(file_location) {
        Ok(file) => {
            let reader = BufReader::new(file);
            read_memento_from_json(reader)
        }
        Err(err) => Err(MementoReadErrors::IO(err)),
    }
}

fn read_memento_from_json<R: Read>(reader: BufReader<R>) -> Result<NewMemento, MementoReadErrors> {
    serde_json::from_reader(reader).map_err(MementoReadErrors::Deserilization)
}

#[cfg(test)]
mod rc_write_mem_tests {
    use std::fs::File;

    use crate::read_memento_from_file;
    use crate::MementoReadErrors;
    use recall_api::models::NewMemento;

    const TEST_MEMENTO_JSON_FILE: &'static str = "./test-data/mem_file.json";

    #[test]
    fn read_memento_from_file_should_be_able_to_read_json_formatted_memento_from_file() {
        let expected_new_memento = NewMemento {
            prompt: "This is a prompt from a file.".to_owned(),
            details: "This is a Memento from a file.".to_owned(),
        };

        let memento = read_memento_from_file(TEST_MEMENTO_JSON_FILE).unwrap();

        assert_eq!(expected_new_memento.prompt, memento.prompt);
        assert_eq!(expected_new_memento.details, memento.details)
    }

    #[test]
    fn read_memento_from_file_should_return_a_file_error_when_the_file_cannot_be_found() {
        let bad_file_location = "./bad_file_name.json".to_owned();
        let actual_err = read_memento_from_file(&bad_file_location).unwrap_err();

        match actual_err {
            MementoReadErrors::IO(_) => assert!(true),
            _ => panic!("Expected: MementoReadErrors::IO, Actual: {}", actual_err),
        }
    }

    #[test]
    fn read_memento_from_file_should_return_a_file_permissions_error_when_the_file_cannot_be_read() {
        use std::process::Command;
        let bad_file_location = "./test-data/bad_permissions_file.json".to_owned();
        let _ = std::fs::remove_file(&bad_file_location);
        let _ = File::create(&bad_file_location);
        Command::new("chmod")
            .arg("-wxr")
            .arg(&bad_file_location)
            .output()
            .expect("failed to execute process");

        let actual_err = read_memento_from_file(&bad_file_location).unwrap_err();
        let _ = std::fs::remove_file(&bad_file_location);

        match actual_err {
            MementoReadErrors::IO(_) => assert!(true),
            _ => panic!("Expected: MementoReadErrors::IO, Actual: {}", actual_err),
        }
    }

    #[test]
    fn read_memento_from_file_should_return_a_deserialization_error_when_the_json_cannot_be_parsed() {
        let bad_file_location = "./test-data/bad_json_file.json".to_owned();
        let _ = std::fs::remove_file(&bad_file_location);
        let _ = File::create(&bad_file_location);

        let actual_err = read_memento_from_file(&bad_file_location).unwrap_err();
        let _ = std::fs::remove_file(&bad_file_location);

        match actual_err {
            MementoReadErrors::Deserilization(_) => assert!(true),
            _ => panic!("Expected: MementoReadErrors::Deserilization, Actual: {}", actual_err),
        }
    }
}
