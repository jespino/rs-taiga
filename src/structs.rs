//
// Taiga
//
pub struct Taiga {
    pub url: String,
    pub token: Option<String>,
}

#[derive(Debug)]
pub struct APIError {
    pub message: String
}

//
// Projects
//

pub struct ProjectsProxy<'a> {
    pub taiga_client: &'a Taiga,
}

pub struct ProjectProxy<'a> {
    pub taiga_client: &'a Taiga,
    pub project_id: i64
}

pub struct Project {
    pub id: i64,
    pub name: String,
}

//
// User Stories
//

pub struct UserStoriesProxy<'a> {
    pub taiga_client: &'a Taiga,
    pub project_id: i64,
}

pub struct UserStoryProxy<'a> {
    pub taiga_client: &'a Taiga,
    pub us_id: i64,
}

pub struct UserStory {
    pub id: i64,
    pub subject: String,
}

