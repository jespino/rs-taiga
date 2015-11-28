use std::collections::HashMap;
use structs::common::{Taiga};

pub struct TasksProxy<'a> {
    pub taiga_client: &'a Taiga,
    pub project_id: Option<i64>,
}

pub struct TaskProxy<'a> {
    pub taiga_client: &'a Taiga,
    pub task_id: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserExtraInfo {
    pub username: String,
    pub full_name_display: String,
    pub photo: String,
    pub big_photo: String,
    pub is_active: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Neighbor {
    pub id: i64,
    #[serde(rename="ref")]
    pub reference: i64,
    pub subject: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Neighbors {
    pub previous: Option<Neighbor>,
    pub next: Option<Neighbor>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskListItem {
    pub id: i64,
    #[serde(rename="ref")]
    pub reference: i64,
    pub user_story: i64,
    pub us_order: i64,
    pub taskboard_order: i64,
    pub is_iocaine: bool,
    pub subject: String,
    pub assigned_to: Option<i64>,
    pub blocked_note: String,
    pub blocked_note_html: String,
    pub comment: String,
    pub created_date: String,
    pub modified_date: String,
    pub finished_date: Option<String>,
    pub is_blocked: bool,
    pub is_closed: bool,
    pub is_voter: bool,
    pub total_voters: i64,
    pub is_watcher: bool,
    pub total_watchers: i64,
    pub external_reference: Option<(String, i64)>,
    pub milestone: Option<i64>,
    pub milestone_slug: Option<String>,
    pub owner: Option<i64>,
    pub project: i64,
    pub status: i64,
    pub tags: Vec<String>,
    pub version: i64,
    pub watchers: Vec<i64>,
    pub status_extra_info: HashMap<String, String>,
    pub assigned_to_extra_info: Option<UserExtraInfo>,
    pub owner_extra_info: Option<UserExtraInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskDetail {
    pub id: i64,
    #[serde(rename="ref")]
    pub reference: i64,
    pub user_story: i64,
    pub us_order: i64,
    pub taskboard_order: i64,
    pub is_iocaine: bool,
    pub subject: String,
    pub description: String,
    pub description_html: String,
    pub assigned_to: Option<i64>,
    pub blocked_note: String,
    pub blocked_note_html: String,
    pub comment: String,
    pub created_date: String,
    pub modified_date: String,
    pub finished_date: Option<String>,
    pub is_blocked: bool,
    pub is_closed: bool,
    pub is_voter: bool,
    pub total_voters: i64,
    pub is_watcher: bool,
    pub total_watchers: i64,
    pub external_reference: Option<(String, i64)>,
    pub milestone: Option<i64>,
    pub milestone_slug: Option<String>,
    pub owner: Option<i64>,
    pub project: i64,
    pub status: i64,
    pub tags: Vec<String>,
    pub version: i64,
    pub watchers: Vec<i64>,
    pub status_extra_info: HashMap<String, String>,
    pub assigned_to_extra_info: Option<UserExtraInfo>,
    pub owner_extra_info: Option<UserExtraInfo>,
    pub neighbors: Neighbors,
}
