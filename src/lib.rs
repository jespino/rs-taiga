extern crate hyper;
extern crate rustc_serialize;

use std::io::Read;

use hyper::{Client, Url};
use hyper::status::StatusCode;
use hyper::method::Method;
use hyper::header::{Headers, ContentType, Authorization};
use rustc_serialize::json::Json;

pub struct APIError {
    pub message: String
}

pub struct Response {
    pub status: StatusCode,
    pub headers: Headers,
    pub data: Json,
}

pub struct Taiga {
    url: String,
    token: Option<String>,
    client: Client,
}

impl Taiga {
    pub fn new(url: String) -> Taiga {
        Taiga {
            url: url,
            token: None,
            client: Client::new()
        }
    }

	pub fn request(self: &mut Taiga, method: Method, url_string: String, body: String) -> Result<Response, APIError> {
        let url = match Url::parse(&url_string) {
            Ok(url) => url,
            Err(err) => return Err(APIError {message: format!("{}", err)})
        };

        let mut req = self.client.request(method, url);

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
            data: Json::from_str(&resp_body).unwrap(),
        };

        return Ok(rest_response);
    }

    pub fn auth(self: &mut Taiga, username: String, password: String) -> Result<Response, APIError> {
        let login_json = format!("{{\"username\": \"{}\", \"password\": \"{}\", \"type\": \"normal\"}}", username, password);
        let login_data = Json::from_str(&login_json).unwrap();
        let url = "".to_string() + &self.url + "/auth";
        match self.request(Method::Post, url, format!("{}", login_data)) {
            Ok(response) => {
                match response.data.find("auth_token") {
                    Some(token) => {
                        let token_type = "Bearer".to_string();
                        self.token = Some(format!("{} {}", token_type, token.as_string().unwrap()));
                        Ok(Response {
                            status: response.status.clone(),
                            headers: response.headers.clone(),
                            data: response.data.clone()
                        })
                    },
                    None => Err(APIError {message: "No auth_token in the reply".to_string()})
                }
            },
            Err(e) => {
                return Err(e)
            }
        }
    }

    pub fn app_auth(self: &mut Taiga, app_token: String) -> Result<Response, APIError> {
        let token_json = format!("{{\"token\": \"{}\"}}", app_token);
        let data = Json::from_str(&token_json).unwrap();
        let token = data.find("token").unwrap().as_string().unwrap().to_string();
        let token_type = "Application".to_string();
		self.token = Some(format!("{} {}", token_type, token));
        return Ok(Response {
            status: StatusCode::Unregistered(0),
            headers: Headers::new(),
            data: data
        });
    }

    pub fn projects(self: &mut Taiga) -> Result<Response, APIError> {
        let url = "".to_string() + &self.url + "/projects";
        return self.request(Method::Get, url, "".to_string());
    }
}
