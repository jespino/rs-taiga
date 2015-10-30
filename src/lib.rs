extern crate hyper;
extern crate rustc_serialize;

use std::io::Read;

use hyper::{Client, Url};
use hyper::status::StatusCode;
use hyper::method::Method;
use hyper::header::{Headers, ContentType, Authorization};
use rustc_serialize::json::Json;

#[derive(Debug)]
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
            data: Json::from_str(&resp_body).unwrap(),
        };

        return Ok(rest_response);
    }

    pub fn auth(self: Taiga, username: String, password: String) -> Result<Taiga, APIError> {
        let login_json = format!("{{\"username\": \"{}\", \"password\": \"{}\", \"type\": \"normal\"}}", username, password);
        let login_data = Json::from_str(&login_json).unwrap();
        let url = "".to_string() + &self.url + "/auth";
        match self.request(Method::Post, url, format!("{}", login_data)) {
            Ok(response) => {
                match response.data.find("auth_token") {
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

    pub fn projects(self: Taiga) -> ProjectsProxy {
        return ProjectsProxy::new(self)
    }
    // pub fn user_stories(self: &mut Taiga) -> UserStoriesProxy {
    //     return UserStoriesProxy::new(self)
    // }
}

pub struct ProjectsProxy {
    taiga_client: Taiga,
}

pub struct ProjectProxy {
    taiga_client: Taiga,
    project_id: i64
}

pub struct Project {
    pub id: i64,
    pub name: String,
}

impl ProjectsProxy {
    pub fn new(taiga_client: Taiga) -> ProjectsProxy {
        ProjectsProxy {
            taiga_client: taiga_client
        }
    }

    pub fn get(self: ProjectsProxy, id: i64) -> ProjectProxy {
        ProjectProxy::new(self.taiga_client, id)
    }

    pub fn run(self: ProjectsProxy) -> Result<Vec<Project>, APIError> {
        let url = format!("{}/projects", self.taiga_client.url);
        match self.taiga_client.request(Method::Get, url, "".to_string()) {
            Ok(response) => {
                match response.data.as_array() {
                    Some(array) => {
						let mut result = Vec::new();
						for item in array {
							let item_data = item.as_object().unwrap();
							result.push(Project {
								            id: item_data.get("id").unwrap().as_i64().unwrap(),
								            name: item_data.get("name").unwrap().as_string().unwrap().to_string(),
									   })
						}
                        Ok(result)
                    },
                    None => Err(APIError {message: "Invalid server response".to_string()})
                }
            },
            Err(e) => {
                return Err(e)
            }
        }
    }
}

impl ProjectProxy {
    pub fn new(taiga_client: Taiga, id: i64) -> ProjectProxy{
        ProjectProxy {
            taiga_client: taiga_client,
            project_id: id
        }
    }

    pub fn run(self: ProjectProxy) -> Result<Project, APIError> {
        let url = format!("{}/projects/{}", self.taiga_client.url, self.project_id);
        match self.taiga_client.request(Method::Get, url, "".to_string()) {
            Ok(response) => {
                match response.data.as_object() {
                    Some(item_data) => {
						Ok(Project {
							id: item_data.get("id").unwrap().as_i64().unwrap(),
							name: item_data.get("name").unwrap().as_string().unwrap().to_string(),
					    })
                    },
                    None => Err(APIError {message: "Invalid server response".to_string()})
                }
            },
            Err(e) => {
                return Err(e)
            }
        }
    }
}
