use hyper::method::Method;
use rustc_serialize::json;

use structs::common::{Taiga, APIError};
use structs::projects::{ProjectsProxy, ProjectProxy, ProjectDetail, ProjectListItem};
use structs::userstories::UserStoriesProxy;

impl<'a> ProjectsProxy<'a> {
    pub fn new(taiga_client: &'a Taiga) -> ProjectsProxy<'a> {
        ProjectsProxy {
            taiga_client: taiga_client
        }
    }

    pub fn get(self: ProjectsProxy<'a>, id: i64) -> ProjectProxy<'a> {
        ProjectProxy::new(self.taiga_client, id)
    }

    pub fn run(self: ProjectsProxy<'a>) -> Result<Vec<ProjectListItem>, APIError> {
        let url = format!("{}/projects", self.taiga_client.url);
        match self.taiga_client.request(Method::Get, url, "".to_string()) {
            Ok(response) => {
                match json::decode(&response.data) {
                    Ok(data) => {
                        let result: Vec<ProjectListItem> = data;
                        Ok(result)
                    },
                    Err(err) => Err(APIError {message: format!("{}", err)})
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

    pub fn run(self: ProjectProxy<'a>) -> Result<ProjectDetail, APIError> {
        let url = format!("{}/projects/{}", self.taiga_client.url, self.project_id);
        match self.taiga_client.request(Method::Get, url, "".to_string()) {
            Ok(response) => {
                match json::decode(&response.data) {
                    Ok(data) => {
                        let result: ProjectDetail = data;
						Ok(result)
                    },
                    Err(err) => Err(APIError {message: format!("{}", err)})
                }
            },
            Err(e) => {
                return Err(e)
            }
        }
    }
}
