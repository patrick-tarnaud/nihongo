use std::{env, fs::File, io::Read};
use env_logger;

use serde::Deserialize;
mod jmdict;

#[derive(Debug, Deserialize, PartialEq)]
struct People {
    person: Vec<Person>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Person {
    lastname: String,
    firstname: String,
    address: Address,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Address {
    street: String,
    postal_code: String,
    city: String,
    country: String,
}


fn load_persons_from_file(file_path: &str) -> Result<People, Box<dyn std::error::Error>> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let people: People = serde_xml_rs::from_str(&contents)?;
    Ok(people)
}

fn main() {
    
    // let people_file_pb = env::current_dir().unwrap().join("resources/people.xml");
    // let people_file = people_file_pb.to_str().expect("Failed to convert current path to string.");
    // let r = match load_persons_from_file(people_file) {
    //     Ok(people) => println!("{people:?}"),
    //     Err(e) => {
    //         eprintln!("Error loading XML: {}", e);
    //         return;
    //     }
    // };
}