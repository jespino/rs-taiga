use hyper::method::Method;

use structs::{Taiga, APIError, UserStoriesProxy, UserStoryProxy, UserStory};

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
