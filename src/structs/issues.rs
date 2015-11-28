use std::collections::HashMap;
use structs::common::{Taiga};

pub struct IssuesProxy<'a> {
    pub taiga_client: &'a Taiga,
    pub project_id: Option<i64>,
}

pub struct IssueProxy<'a> {
    pub taiga_client: &'a Taiga,
    pub issue_id: i64,
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
pub struct IssueListItem {
    pub id: i64,
    #[serde(rename="ref")]
    pub reference: i64,
    pub subject: String,
    pub assigned_to: Option<i64>,
    pub blocked_note: String,
    pub blocked_note_html: String,
    pub comment: String,
    pub created_date: String,
    pub modified_date: String,
    pub finished_date: Option<String>,
    pub generated_user_stories: Vec<i64>,
    pub is_blocked: bool,
    pub is_closed: bool,
    pub is_voter: bool,
    pub total_voters: i64,
    pub is_watcher: bool,
    pub total_watchers: i64,
    pub external_reference: Option<(String, i64)>,
    pub milestone: Option<i64>,
    pub owner: Option<i64>,
    pub project: i64,
    pub status: i64,
    pub severity: i64,
    pub priority: i64,
    #[serde(rename="type")]
    pub issue_type: i64,
    pub tags: Vec<String>,
    pub version: i64,
    pub watchers: Vec<i64>,
    pub status_extra_info: HashMap<String, String>,
    pub assigned_to_extra_info: Option<UserExtraInfo>,
    pub owner_extra_info: Option<UserExtraInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IssueDetail {
    pub id: i64,
    #[serde(rename="ref")]
    pub reference: i64,
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
    pub generated_user_stories: Vec<i64>,
    pub is_blocked: bool,
    pub is_closed: bool,
    pub is_voter: bool,
    pub total_voters: i64,
    pub is_watcher: bool,
    pub total_watchers: i64,
    pub external_reference: Option<(String, i64)>,
    pub milestone: Option<i64>,
    pub owner: Option<i64>,
    pub project: i64,
    pub status: i64,
    pub severity: i64,
    pub priority: i64,
    #[serde(rename="type")]
    pub issue_type: i64,
    pub tags: Vec<String>,
    pub version: i64,
    pub watchers: Vec<i64>,
    pub status_extra_info: HashMap<String, String>,
    pub assigned_to_extra_info: Option<UserExtraInfo>,
    pub owner_extra_info: Option<UserExtraInfo>,
    pub neighbors: Neighbors,
}
