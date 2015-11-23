use hyper::method::Method;
use serde_json;

use structs::common::{Taiga, APIError, ObjectType, DeleteProxy};
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

    pub fn delete(self: ProjectsProxy<'a>, id: i64) -> DeleteProxy<'a> {
        DeleteProxy::new(self.taiga_client, ObjectType::Project, id)
    }

    pub fn run(self: ProjectsProxy<'a>) -> Result<Vec<ProjectListItem>, APIError> {
        let url = format!("{}/projects", self.taiga_client.url);
        match self.taiga_client.request(Method::Get, url, "".to_string()) {
            Ok(response) => {
                match serde_json::from_str(&response.data) {
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

    pub fn delete(self: ProjectProxy<'a>) -> DeleteProxy<'a> {
        DeleteProxy::new(self.taiga_client, ObjectType::Project, self.project_id)
    }

    pub fn run(self: ProjectProxy<'a>) -> Result<ProjectDetail, APIError> {
        let url = format!("{}/projects/{}", self.taiga_client.url, self.project_id);
        match self.taiga_client.request(Method::Get, url, "".to_string()) {
            Ok(response) => {
                match serde_json::from_str(&response.data) {
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

#[cfg(test)]
mod tests {
    use structs::common::{Taiga, ObjectType};
    use structs::projects::{ProjectsProxy, ProjectProxy};

    #[test]
    fn project_userstories() {
        let taiga = Taiga::new("http://dummy-url.com".to_string());
        let result = ProjectProxy::new(&taiga, 10).userstories();
        assert_eq!(result.project_id.unwrap(), 10);
    }

    #[test]
    fn project_delete() {
        let taiga = Taiga::new("http://dummy-url.com".to_string());
        let result = ProjectProxy::new(&taiga, 10).delete();
        assert_eq!(result.object_type, ObjectType::Project);
        assert_eq!(result.object_id, 10);
    }

    #[test]
    fn projects_get() {
        let taiga = Taiga::new("http://dummy-url.com".to_string());
        let result = ProjectsProxy::new(&taiga).get(10);
        assert_eq!(result.project_id, 10);
    }

    #[test]
    fn projects_delete() {
        let taiga = Taiga::new("http://dummy-url.com".to_string());
        let result = ProjectsProxy::new(&taiga).delete(10);
        assert_eq!(result.object_type, ObjectType::Project);
        assert_eq!(result.object_id, 10);
    }
}
