use std::collections::HashMap;
use structs::common::{Taiga};

pub struct UserStoriesProxy<'a> {
    pub taiga_client: &'a Taiga,
    pub project_id: Option<i64>,
}

pub struct UserStoryProxy<'a> {
    pub taiga_client: &'a Taiga,
    pub us_id: i64,
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
pub struct UserStoryListItem {
    pub id: i64,
    #[serde(rename="ref")]
    pub reference: i64,
    pub subject: String,
    pub assigned_to: Option<i64>,
    pub backlog_order: i64,
    pub kanban_order: i64,
    pub sprint_order: i64,
    pub blocked_note: String,
    pub blocked_note_html: String,
    pub client_requirement: bool,
    pub team_requirement: bool,
    pub comment: String,
    pub created_date: String,
    pub modified_date: String,
    pub finish_date: Option<String>,
    pub generated_from_issue: Option<i64>,
    pub is_blocked: bool,
    pub is_closed: bool,
    pub is_voter: bool,
    pub total_voters: i64,
    pub is_watcher: bool,
    pub total_watchers: i64,
    pub external_reference: Option<(String, i64)>,
    pub milestone: Option<i64>,
    pub milestone_name: Option<String>,
    pub milestone_slug: Option<String>,
    pub origin_issue: Option<i64>,
    pub owner: Option<i64>,
    pub points: HashMap<i64, i64>,
    pub project: i64,
    pub status: i64,
    pub tags: Vec<String>,
    pub total_points: f64,
    pub version: i64,
    pub watchers: Vec<i64>,
    pub status_extra_info: HashMap<String, String>,
    pub assigned_to_extra_info: Option<UserExtraInfo>,
    pub owner_extra_info: Option<UserExtraInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserStoryDetail {
    pub id: i64,
    #[serde(rename="ref")]
    pub reference: i64,
    pub subject: String,
    pub description: String,
    pub description_html: String,
    pub assigned_to: Option<i64>,
    pub backlog_order: i64,
    pub kanban_order: i64,
    pub sprint_order: i64,
    pub blocked_note: String,
    pub blocked_note_html: String,
    pub client_requirement: bool,
    pub team_requirement: bool,
    pub comment: String,
    pub created_date: String,
    pub modified_date: String,
    pub finish_date: Option<String>,
    pub generated_from_issue: Option<i64>,
    pub is_blocked: bool,
    pub is_closed: bool,
    pub is_voter: bool,
    pub total_voters: i64,
    pub is_watcher: bool,
    pub total_watchers: i64,
    pub external_reference: Option<(String, i64)>,
    pub milestone: Option<i64>,
    pub milestone_name: Option<String>,
    pub milestone_slug: Option<String>,
    pub origin_issue: Option<i64>,
    pub owner: Option<i64>,
    pub points: HashMap<i64, i64>,
    pub project: i64,
    pub status: i64,
    pub tags: Vec<String>,
    pub total_points: f64,
    pub version: i64,
    pub watchers: Vec<i64>,
    pub status_extra_info: HashMap<String, String>,
    pub assigned_to_extra_info: Option<UserExtraInfo>,
    pub owner_extra_info: Option<UserExtraInfo>,
    pub neighbors: Neighbors,
}
