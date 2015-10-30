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

    pub fn projects(self: &Taiga) -> ProjectsProxy {
        return ProjectsProxy::new(self)
    }
    // pub fn user_stories(self: &mut Taiga) -> UserStoriesProxy {
    //     return UserStoriesProxy::new(self)
    // }
}

//
// Projects
//

pub struct ProjectsProxy<'a> {
    taiga_client: &'a Taiga,
}

pub struct ProjectProxy<'a> {
    taiga_client: &'a Taiga,
    project_id: i64
}

pub struct Project {
    pub id: i64,
    pub name: String,
}

impl<'a> ProjectsProxy<'a> {
    pub fn new(taiga_client: &'a Taiga) -> ProjectsProxy<'a> {
        ProjectsProxy {
            taiga_client: taiga_client
        }
    }

    pub fn get(self: ProjectsProxy<'a>, id: i64) -> ProjectProxy<'a> {
        ProjectProxy::new(self.taiga_client, id)
    }

    pub fn run(self: ProjectsProxy<'a>) -> Result<Vec<Project>, APIError> {
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

impl<'a> ProjectProxy<'a> {
    pub fn new(taiga_client: &'a Taiga, id: i64) -> ProjectProxy<'a>{
        ProjectProxy {
            taiga_client: taiga_client,
            project_id: id
        }
    }

    pub fn userstories(self: ProjectProxy<'a>) -> UserStoriesProxy<'a> {
        UserStoriesProxy::new(self.taiga_client, self.project_id)
    }

    pub fn run(self: ProjectProxy<'a>) -> Result<Project, APIError> {
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

//
// User Stories
//

pub struct UserStoriesProxy<'a> {
    taiga_client: &'a Taiga,
    project_id: i64,
}

pub struct UserStoryProxy<'a> {
    taiga_client: &'a Taiga,
    us_id: i64,
}

pub struct UserStory {
    pub id: i64,
    pub subject: String,
}

impl<'a> UserStoriesProxy<'a> {
    pub fn new(taiga_client: &'a Taiga, project_id: i64) -> UserStoriesProxy<'a> {
        UserStoriesProxy {
            taiga_client: taiga_client,
            project_id: project_id
        }
    }

    pub fn get(self: UserStoriesProxy<'a>, id: i64) -> UserStoryProxy<'a> {
        UserStoryProxy::new(self.taiga_client, id)
    }

    pub fn run(self: UserStoriesProxy<'a>) -> Result<Vec<UserStory>, APIError> {
        let url = format!("{}/userstories?project_id={}", self.taiga_client.url, self.project_id);
        match self.taiga_client.request(Method::Get, url, "".to_string()) {
            Ok(response) => {
                match response.data.as_array() {
                    Some(array) => {
						let mut result = Vec::new();
						for item in array {
							let item_data = item.as_object().unwrap();
							result.push(UserStory {
								            id: item_data.get("id").unwrap().as_i64().unwrap(),
								            subject: item_data.get("subject").unwrap().as_string().unwrap().to_string(),
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

impl<'a> UserStoryProxy<'a> {
    pub fn new(taiga_client: &'a Taiga, us_id: i64) -> UserStoryProxy<'a>{
        UserStoryProxy {
            taiga_client: taiga_client,
            us_id: us_id
        }
    }

    pub fn run(self: UserStoryProxy<'a>) -> Result<UserStory, APIError> {
        let url = format!("{}/userstories/{}", self.taiga_client.url, self.us_id);
        match self.taiga_client.request(Method::Get, url, "".to_string()) {
            Ok(response) => {
                match response.data.as_object() {
                    Some(item_data) => {
						Ok(UserStory {
							id: item_data.get("id").unwrap().as_i64().unwrap(),
							subject: item_data.get("subject").unwrap().as_string().unwrap().to_string(),
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
