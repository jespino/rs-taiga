extern crate taiga;

use std::process;
use taiga::Taiga;

fn main() {
    let mut taiga = Taiga::new("http://localhost:8000/api/v1".to_string());

    let result = taiga.auth("admin".to_string(), "123123".to_string());
    if result.is_err() {
        println!("{}", result.err().unwrap().message);
        process::exit(1);
    }

    match taiga.projects() {
        Ok(response) => {
            for project in response.data.as_array().unwrap() {
                println!("{}", project.find("name").unwrap().as_string().unwrap());
            }
        },
        Err(e) => println!("{}", e.message)
    }
}
