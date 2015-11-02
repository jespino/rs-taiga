use hyper::method::Method;
use rustc_serialize::json;

use structs::common::{Taiga, APIError};
use structs::userstories::{UserStoriesProxy, UserStoryProxy, UserStory};

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
