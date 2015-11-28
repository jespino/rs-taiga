use hyper::method::Method;
use serde_json;

use structs::common::{Taiga, APIError, ObjectType, DeleteProxy};
use structs::tasks::{TasksProxy, TaskProxy, TaskListItem, TaskDetail};

impl<'a> TasksProxy<'a> {
    pub fn new(taiga_client: &'a Taiga, project_id: i64) -> TasksProxy<'a> {
        TasksProxy {
            taiga_client: taiga_client,
            project_id: Some(project_id)
        }
    }

    pub fn new_all(taiga_client: &'a Taiga) -> TasksProxy<'a> {
        TasksProxy {
            taiga_client: taiga_client,
            project_id: None
        }
    }

    pub fn get(self: TasksProxy<'a>, id: i64) -> TaskProxy<'a> {
        TaskProxy::new(self.taiga_client, id)
    }

    pub fn delete(self: TasksProxy<'a>, id: i64) -> DeleteProxy<'a> {
        DeleteProxy::new(self.taiga_client, ObjectType::Task, id)
    }

    pub fn run(self: TasksProxy<'a>) -> Result<Vec<TaskListItem>, APIError> {
        let url = match self.project_id {
            Some(project_id) => format!("{}/tasks?project_id={}", self.taiga_client.url, project_id),
            None => format!("{}/tasks", self.taiga_client.url)
        };
        match self.taiga_client.request(Method::Get, url, "".to_string()) {
            Ok(response) => {
                match serde_json::from_str(&response.data) {
                    Ok(data) => {
                        let result: Vec<TaskListItem> = data;
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

impl<'a> TaskProxy<'a> {
    pub fn new(taiga_client: &'a Taiga, task_id: i64) -> TaskProxy<'a>{
        TaskProxy {
            taiga_client: taiga_client,
            task_id: task_id
        }
    }

    pub fn delete(self: TaskProxy<'a>) -> DeleteProxy<'a> {
        DeleteProxy::new(self.taiga_client, ObjectType::Task, self.task_id)
    }

    pub fn run(self: TaskProxy<'a>) -> Result<TaskDetail, APIError> {
        let url = format!("{}/tasks/{}", self.taiga_client.url, self.task_id);
        match self.taiga_client.request(Method::Get, url, "".to_string()) {
            Ok(response) => {
                match serde_json::from_str(&response.data) {
                    Ok(data) => {
                        let result: TaskDetail = data;
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
    use structs::tasks::{TasksProxy, TaskProxy};

    #[test]
    fn tasks_get() {
        let taiga = Taiga::new("http://dummy-url.com".to_string());
        let result = TasksProxy::new(&taiga, 10).get(10);
        assert_eq!(result.task_id, 10);
    }

    #[test]
    fn tasks_delete() {
        let taiga = Taiga::new("http://dummy-url.com".to_string());
        let result = TasksProxy::new(&taiga, 5).delete(10);
        assert_eq!(result.object_type, ObjectType::Task);
        assert_eq!(result.object_id, 10);
    }

    #[test]
    fn task_delete() {
        let taiga = Taiga::new("http://dummy-url.com".to_string());
        let result = TaskProxy::new(&taiga, 10).delete();
        assert_eq!(result.object_type, ObjectType::Task);
        assert_eq!(result.object_id, 10);
    }
}
