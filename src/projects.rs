use hyper::method::Method;

use structs::{Taiga, APIError, ProjectsProxy, ProjectProxy, Project, UserStoriesProxy};

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
