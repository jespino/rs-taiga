extern crate hyper;
extern crate rustc_serialize;
extern crate time;

pub mod structs;
mod projects;
mod userstories;
mod common;

use std::io::Read;

use hyper::{Client, Url};
use hyper::status::StatusCode;
use hyper::method::Method;
use hyper::header::{Headers, ContentType, Authorization};
use rustc_serialize::json::Json;

use structs::common::{Taiga, APIError};
use structs::projects::ProjectsProxy;

pub struct Response {
    pub status: StatusCode,
    pub headers: Headers,
    pub data: String,
}

impl Taiga {
    pub fn new(url: String) -> Taiga {
        Taiga {
            url: url,
            token: None,
        }
    }

	pub fn request(self: &Taiga, method: Method, url_string: String, body: String) -> Result<Response, APIError> {
        let url = match Url::parse(&url_string) {
            Ok(url) => url,
            Err(err) => return Err(APIError {message: format!("{}", err)})
        };

        let client = Client::new();
        let mut req = client.request(method, url);

		req = match self.token.clone() {
			Some(token) => req.header(Authorization(token)),
            None => req
		};

        req = req.header(ContentType::json());
        req = req.body(body.as_bytes());

        let mut resp = match req.send() {
            Ok(resp) => resp,
            Err(err) => return Err(APIError {message: format!("{}", err)})
        };

        let mut resp_body = String::new();
        match resp.read_to_string(&mut resp_body) {
            Ok(_) => (),
            Err(err) => return Err(APIError {message: format!("{}", err)})
        };

        let rest_response = Response {
            status: resp.status,
            headers: resp.headers.clone(),
            data: resp_body,
        };

        return Ok(rest_response);
    }

    pub fn auth(self: Taiga, username: String, password: String) -> Result<Taiga, APIError> {
        let login_json = format!("{{\"username\": \"{}\", \"password\": \"{}\", \"type\": \"normal\"}}", username, password);
        let login_data = Json::from_str(&login_json).unwrap();
        let url = "".to_string() + &self.url + "/auth";
        match self.request(Method::Post, url, format!("{}", login_data)) {
            Ok(response) => {
                match Json::from_str(&response.data).unwrap().find("auth_token") {
                    Some(token) => {
                        Ok(Taiga {url: self.url, token: Some(format!("Bearer {}", token.as_string().unwrap()))})
                    },
                    None => Err(APIError {message: "No auth_token in the reply".to_string()})
                }
            },
            Err(e) => {
                return Err(e)
            }
        }
    }

    pub fn app_auth(self: Taiga, app_token: String) -> Result<Taiga, APIError> {
        return Ok(Taiga {url: self.url, token: Some(format!("Application {}", app_token))});
    }

    pub fn projects(self: &Taiga) -> ProjectsProxy {
        return ProjectsProxy::new(self)
    }
    // pub fn user_stories(self: &mut Taiga) -> UserStoriesProxy {
    //     return UserStoriesProxy::new(self)
    // }
}
