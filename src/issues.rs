use hyper::method::Method;
use serde_json;

use structs::common::{Taiga, APIError, ObjectType, DeleteProxy};
use structs::issues::{IssuesProxy, IssueProxy, IssueListItem, IssueDetail};

impl<'a> IssuesProxy<'a> {
    pub fn new(taiga_client: &'a Taiga, project_id: i64) -> IssuesProxy<'a> {
        IssuesProxy {
            taiga_client: taiga_client,
            project_id: Some(project_id)
        }
    }

    pub fn new_all(taiga_client: &'a Taiga) -> IssuesProxy<'a> {
        IssuesProxy {
            taiga_client: taiga_client,
            project_id: None
        }
    }

    pub fn get(self: IssuesProxy<'a>, id: i64) -> IssueProxy<'a> {
        IssueProxy::new(self.taiga_client, id)
    }

    pub fn delete(self: IssuesProxy<'a>, id: i64) -> DeleteProxy<'a> {
        DeleteProxy::new(self.taiga_client, ObjectType::Issue, id)
    }

    pub fn run(self: IssuesProxy<'a>) -> Result<Vec<IssueListItem>, APIError> {
        let url = match self.project_id {
            Some(project_id) => format!("{}/issues?project_id={}", self.taiga_client.url, project_id),
            None => format!("{}/issues", self.taiga_client.url)
        };
        match self.taiga_client.request(Method::Get, url, "".to_string()) {
            Ok(response) => {
                match serde_json::from_str(&response.data) {
                    Ok(data) => {
                        let result: Vec<IssueListItem> = data;
                        Ok(result)
                    },
                    Err(_) => Err(APIError {message: "Invalid server response".to_string()})
                    // Err(e) => Err(APIError {message: format!("{}", e)})
                }
            },
            Err(e) => {
                return Err(e)
            }
        }
    }
}

impl<'a> IssueProxy<'a> {
    pub fn new(taiga_client: &'a Taiga, issue_id: i64) -> IssueProxy<'a>{
        IssueProxy {
            taiga_client: taiga_client,
            issue_id: issue_id
        }
    }

    pub fn delete(self: IssueProxy<'a>) -> DeleteProxy<'a> {
        DeleteProxy::new(self.taiga_client, ObjectType::Issue, self.issue_id)
    }

    pub fn run(self: IssueProxy<'a>) -> Result<IssueDetail, APIError> {
        let url = format!("{}/issues/{}", self.taiga_client.url, self.issue_id);
        match self.taiga_client.request(Method::Get, url, "".to_string()) {
            Ok(response) => {
                match serde_json::from_str(&response.data) {
                    Ok(data) => {
                        let result: IssueDetail = data;
                        Ok(result)
                    },
                    Err(_) => Err(APIError {message: "Invalid server response".to_string()})
                    // Err(e) => Err(APIError {message: format!("{}", e)})
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
    use structs::issues::{IssuesProxy, IssueProxy};

    #[test]
    fn issues_get() {
        let taiga = Taiga::new("http://dummy-url.com".to_string());
        let result = IssuesProxy::new(&taiga, 10).get(10);
        assert_eq!(result.issue_id, 10);
    }

    #[test]
    fn issues_delete() {
        let taiga = Taiga::new("http://dummy-url.com".to_string());
        let result = IssuesProxy::new(&taiga, 5).delete(10);
        assert_eq!(result.object_type, ObjectType::Issue);
        assert_eq!(result.object_id, 10);
    }

    #[test]
    fn issue_delete() {
        let taiga = Taiga::new("http://dummy-url.com".to_string());
        let result = IssueProxy::new(&taiga, 10).delete();
        assert_eq!(result.object_type, ObjectType::Issue);
        assert_eq!(result.object_id, 10);
    }
}
