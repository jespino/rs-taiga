extern crate taiga;

use std::process;
use taiga::Taiga;

fn main() {
    let taiga = Taiga::new("http://localhost:8000/api/v1".to_string());

    let taiga_logged = match taiga.auth("admin".to_string(), "123123".to_string()) {
        Ok(taiga_logged) => taiga_logged,
        Err(api_error) => {
            println!("{}", api_error.message);
            process::exit(1);
        }
    };

    match taiga_logged.projects().run() {
        Ok(response) => {
            for project in response {
                println!("{} - {}",
                    project.id,
                    project.name,
                );
            }
        },
        Err(e) => println!("{}", e.message)
    }

    // match taiga.projects().get(1).run() {
    //     Ok(response) => {
    //         for user_story in response.data.as_array().unwrap() {
    //             println!("{}", user_story.find("subject").unwrap().as_string().unwrap());
    //         }
    //     },
    //     Err(e) => println!("{}", e.message)
    // }
}
