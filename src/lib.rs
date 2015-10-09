extern crate hyper;
extern crate rustc_serialize;

use std::io::Read;

use hyper::Client;
use hyper::header::ContentType;
use hyper::header::Authorization;
use rustc_serialize::json::Json;

pub struct APIError {
    pub message: String
}

pub struct Taiga {
    url: String,
    token: String,
    token_type: String,
    client: Client,
}

impl Taiga {
    pub fn new(url: String) -> Taiga {
        Taiga {
            url: url,
            token: String::new(),
            token_type: String::new(),
            client: Client::new()
        }
    }

    pub fn resolve(self: &mut Taiga, name: &str) -> String {
        if name == "auth" {
            return "".to_string() + &self.url + "/auth";
        } else if name == "projects" {
            return "".to_string() + &self.url + "/projects";
        } else {
            return "".to_string() + &self.url;
        }
    }

    pub fn get(self: &mut Taiga, url: String) -> Result<Json, APIError> {
        let auth_token = format!("{} {}", self.token_type, self.token);
        match self.client.get(&url).header(ContentType::json()).header(Authorization(auth_token)).send() {
            Ok(mut response) =>  {
                let mut body = String::new();
                response.read_to_string(&mut body).unwrap();
                let data: Json = Json::from_str(&body).unwrap();
                Ok(data)
            },
            Err(_) => Err(APIError {message: "TODO".to_string()})
        }
    }

    pub fn post(self: &mut Taiga, url: String, data: Json) -> Result<Json, APIError> {
        let encoded_data = format!("{}", data.pretty());
        match self.client.post(&url).header(ContentType::json()).body(&encoded_data).send() {
            Ok(mut response) =>  {
                let mut body = String::new();
                response.read_to_string(&mut body).unwrap();
                let data: Json = Json::from_str(&body).unwrap();
                Ok(data)
            },
            Err(_) => Err(APIError {message: "TODO".to_string()})
        }
    }

    pub fn auth(self: &mut Taiga, username: String, password: String) -> Result<Json, APIError> {
        let login_json = format!("{{\"username\": \"{}\", \"password\": \"{}\", \"type\": \"normal\"}}", username, password);
        let login_data = Json::from_str(&login_json).unwrap();
        let url = self.resolve("auth");
        match self.post(url, login_data) {
            Ok(data) => {
                match data.find("auth_token") {
                    Some(token) => {
                        self.token = token.as_string().unwrap().to_string();
                        self.token_type = "Bearer".to_string();
                        Ok(data.clone())
                    },
                    None => Err(APIError {message: "No auth_token in the reply".to_string()})
                }
            },
            Err(e) => {
                return Err(e)
            }
        }
    }

    pub fn app_auth(self: &mut Taiga, app_token: String) -> Result<Json, APIError> {
        let token_json = format!("{{\"token\": \"{}\"}}", app_token);
        let data = Json::from_str(&token_json).unwrap();
        self.token = data.find("token").unwrap().as_string().unwrap().to_string();
        self.token_type = "Application".to_string();
        return Ok(data);
    }

    pub fn projects(self: &mut Taiga) -> Result<Json, APIError> {
        let url = self.resolve("projects");
        return self.get(url)
    }
}
