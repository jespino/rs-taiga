use structs::common::{Taiga};

pub struct UserStoriesProxy<'a> {
    pub taiga_client: &'a Taiga,
    pub project_id: i64,
}

pub struct UserStoryProxy<'a> {
    pub taiga_client: &'a Taiga,
    pub us_id: i64,
}

#[derive(RustcEncodable, RustcDecodable, Debug)]
pub struct UserStory {
    pub id: i64,
    pub subject: String,
}
