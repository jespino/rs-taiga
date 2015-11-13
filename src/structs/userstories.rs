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


#[derive(RustcEncodable, RustcDecodable, Debug)]
pub struct Neighbor {
    pub id: i64,
    pub reference: i64,
    pub subject: String,
}

#[derive(RustcEncodable, RustcDecodable, Debug)]
pub struct UserStoryListItem {
    pub id: i64,
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
    pub finish_date: String,
    pub generated_from_issue: Option<i64>,
    pub is_archived: bool,
    pub is_blocked: bool,
    pub is_closed: bool,
    pub milestone: i64,
    pub milestone_name: String,
    pub milestone_slug: String,
    pub origin_issue: Option<i64>,
    pub owner: i64,
    pub points: HashMap<i64, i64>,
    pub project: i64,
    pub status: i64,
    pub tags: Vec<String>,
    pub total_points: f64,
    pub version: i64,
    pub watchers: Vec<i64>,
    pub status_extra_info: HashMap<String, String>,
    pub assigend_to_extra_info: HashMap<String, String>,
}

#[derive(RustcEncodable, RustcDecodable, Debug)]
pub struct UserStoryDetail {
    pub id: i64,
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
    pub finish_date: String,
    pub generated_from_issue: Option<i64>,
    pub is_archived: bool,
    pub is_blocked: bool,
    pub is_closed: bool,
    pub milestone: i64,
    pub milestone_name: String,
    pub milestone_slug: String,
    pub origin_issue: Option<i64>,
    pub owner: i64,
    pub points: HashMap<i64, i64>,
    pub project: i64,
    pub status: i64,
    pub tags: Vec<String>,
    pub total_points: f64,
    pub version: i64,
    pub watchers: Vec<i64>,
    pub status_extra_info: HashMap<String, String>,
    pub assigend_to_extra_info: HashMap<String, String>,
    pub neighbors: HashMap<String, Neighbor>,
}
