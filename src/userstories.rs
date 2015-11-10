use hyper::method::Method;
use rustc_serialize::json;

use structs::common::{Taiga, APIError, ObjectType, DeleteProxy};
use structs::userstories::{UserStoriesProxy, UserStoryProxy, UserStory};

impl<'a> UserStoriesProxy<'a> {
    pub fn new(taiga_client: &'a Taiga, project_id: i64) -> UserStoriesProxy<'a> {
        UserStoriesProxy {
            taiga_client: taiga_client,
            project_id: Some(project_id)
        }
    }

    pub fn new_all(taiga_client: &'a Taiga) -> UserStoriesProxy<'a> {
        UserStoriesProxy {
            taiga_client: taiga_client,
            project_id: None
        }
    }

    pub fn get(self: UserStoriesProxy<'a>, id: i64) -> UserStoryProxy<'a> {
        UserStoryProxy::new(self.taiga_client, id)
    }

    pub fn delete(self: UserStoriesProxy<'a>, id: i64) -> DeleteProxy<'a> {
        DeleteProxy::new(self.taiga_client, ObjectType::UserStory, id)
    }

    pub fn run(self: UserStoriesProxy<'a>) -> Result<Vec<UserStory>, APIError> {
        let url = match self.project_id {
            Some(project_id) => format!("{}/userstories?project_id={}", self.taiga_client.url, project_id),
            None => format!("{}/userstories", self.taiga_client.url)
        };
        match self.taiga_client.request(Method::Get, url, "".to_string()) {
            Ok(response) => {
                match json::decode(&response.data) {
                    Ok(data) => {
						let result: Vec<UserStory> = data;
                        Ok(result)
                    },
                    Err(_) => Err(APIError {message: "Invalid server response".to_string()})
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

    pub fn delete(self: UserStoryProxy<'a>) -> DeleteProxy<'a> {
        DeleteProxy::new(self.taiga_client, ObjectType::UserStory, self.us_id)
    }

    pub fn run(self: UserStoryProxy<'a>) -> Result<UserStory, APIError> {
        let url = format!("{}/userstories/{}", self.taiga_client.url, self.us_id);
        match self.taiga_client.request(Method::Get, url, "".to_string()) {
            Ok(response) => {
                match json::decode(&response.data) {
                    Ok(data) => {
                        let result: UserStory = data;
						Ok(result)
                    },
                    Err(_) => Err(APIError {message: "Invalid server response".to_string()})
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
    use structs::userstories::{UserStoriesProxy, UserStoryProxy};

    #[test]
    fn userstories_get() {
        let taiga = Taiga::new("http://dummy-url.com".to_string());
        let result = UserStoriesProxy::new(&taiga, 10).get(10);
        assert_eq!(result.us_id, 10);
    }

    #[test]
    fn userstories_delete() {
        let taiga = Taiga::new("http://dummy-url.com".to_string());
        let result = UserStoriesProxy::new(&taiga, 5).delete(10);
        assert_eq!(result.object_type, ObjectType::UserStory);
        assert_eq!(result.object_id, 10);
    }

    #[test]
    fn userstory_delete() {
        let taiga = Taiga::new("http://dummy-url.com".to_string());
        let result = UserStoryProxy::new(&taiga, 10).delete();
        assert_eq!(result.object_type, ObjectType::UserStory);
        assert_eq!(result.object_id, 10);
    }
}
