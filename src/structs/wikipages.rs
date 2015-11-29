use structs::common::{Taiga};

pub struct WikiPagesProxy<'a> {
    pub taiga_client: &'a Taiga,
    pub project_id: Option<i64>,
}

pub struct WikiPageProxy<'a> {
    pub taiga_client: &'a Taiga,
    pub wikipage_id: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WikiPageListItem {
    pub id: i64,
    pub slug: String,
    pub html: String,
    pub content: String,
    pub is_watcher: bool,
    pub total_watchers: i64,
    pub editions: i64,
    pub version: i64,
    pub project: i64,
    pub owner: Option<i64>,
    pub last_modifier: Option<i64>,
    pub created_date: String,
    pub modified_date: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WikiPageDetail {
    pub id: i64,
    pub slug: String,
    pub html: String,
    pub content: String,
    pub is_watcher: bool,
    pub total_watchers: i64,
    pub editions: i64,
    pub version: i64,
    pub project: i64,
    pub owner: Option<i64>,
    pub last_modifier: Option<i64>,
    pub created_date: String,
    pub modified_date: String,
}
