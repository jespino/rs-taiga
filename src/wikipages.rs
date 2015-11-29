use hyper::method::Method;
use serde_json;

use structs::common::{Taiga, APIError, ObjectType, DeleteProxy};
use structs::wikipages::{WikiPagesProxy, WikiPageProxy, WikiPageListItem, WikiPageDetail};

impl<'a> WikiPagesProxy<'a> {
    pub fn new(taiga_client: &'a Taiga, project_id: i64) -> WikiPagesProxy<'a> {
        WikiPagesProxy {
            taiga_client: taiga_client,
            project_id: Some(project_id)
        }
    }

    pub fn new_all(taiga_client: &'a Taiga) -> WikiPagesProxy<'a> {
        WikiPagesProxy {
            taiga_client: taiga_client,
            project_id: None
        }
    }

    pub fn get(self: WikiPagesProxy<'a>, id: i64) -> WikiPageProxy<'a> {
        WikiPageProxy::new(self.taiga_client, id)
    }

    pub fn delete(self: WikiPagesProxy<'a>, id: i64) -> DeleteProxy<'a> {
        DeleteProxy::new(self.taiga_client, ObjectType::WikiPage, id)
    }

    pub fn run(self: WikiPagesProxy<'a>) -> Result<Vec<WikiPageListItem>, APIError> {
        let url = match self.project_id {
            Some(project_id) => format!("{}/wiki?project_id={}", self.taiga_client.url, project_id),
            None => format!("{}/wiki", self.taiga_client.url)
        };
        match self.taiga_client.request(Method::Get, url, "".to_string()) {
            Ok(response) => {
                match serde_json::from_str(&response.data) {
                    Ok(data) => {
                        let result: Vec<WikiPageListItem> = data;
                        Ok(result)
                    },
                    // Err(_) => Err(APIError {message: "Invalid server response".to_string()})
                    Err(e) => Err(APIError {message: format!("{}", e)})
                }
            },
            Err(e) => {
                return Err(e)
            }
        }
    }
}

impl<'a> WikiPageProxy<'a> {
    pub fn new(taiga_client: &'a Taiga, wikipage_id: i64) -> WikiPageProxy<'a>{
        WikiPageProxy {
            taiga_client: taiga_client,
            wikipage_id: wikipage_id
        }
    }

    pub fn delete(self: WikiPageProxy<'a>) -> DeleteProxy<'a> {
        DeleteProxy::new(self.taiga_client, ObjectType::WikiPage, self.wikipage_id)
    }

    pub fn run(self: WikiPageProxy<'a>) -> Result<WikiPageDetail, APIError> {
        let url = format!("{}/wiki/{}", self.taiga_client.url, self.wikipage_id);
        match self.taiga_client.request(Method::Get, url, "".to_string()) {
            Ok(response) => {
                match serde_json::from_str(&response.data) {
                    Ok(data) => {
                        let result: WikiPageDetail = data;
                        Ok(result)
                    },
                    // Err(_) => Err(APIError {message: "Invalid server response".to_string()})
                    Err(e) => Err(APIError {message: format!("{}", e)})
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
    use structs::wikipages::{WikiPagesProxy, WikiPageProxy};

    #[test]
    fn wikipages_get() {
        let taiga = Taiga::new("http://dummy-url.com".to_string());
        let result = WikiPagesProxy::new(&taiga, 10).get(10);
        assert_eq!(result.wikipage_id, 10);
    }

    #[test]
    fn wikipages_delete() {
        let taiga = Taiga::new("http://dummy-url.com".to_string());
        let result = WikiPagesProxy::new(&taiga, 5).delete(10);
        assert_eq!(result.object_type, ObjectType::WikiPage);
        assert_eq!(result.object_id, 10);
    }

    #[test]
    fn wikipage_delete() {
        let taiga = Taiga::new("http://dummy-url.com".to_string());
        let result = WikiPageProxy::new(&taiga, 10).delete();
        assert_eq!(result.object_type, ObjectType::WikiPage);
        assert_eq!(result.object_id, 10);
    }
}
