use structs::common::{Taiga, CustomAttribute, IssueStatus, IssueType, USStatus, TaskStatus, Priority, Severity, Role, Points};
use std::collections::HashMap;

pub struct ProjectsProxy<'a> {
    pub taiga_client: &'a Taiga,
}

pub struct ProjectProxy<'a> {
    pub taiga_client: &'a Taiga,
    pub project_id: i64
}

#[derive(RustcEncodable, RustcDecodable, Debug)]
pub struct ProjectMember {
    pub id: i64,
    pub username: String,
    pub full_name: String,
    pub full_name_display: String,
    pub color: String,
    pub photo: String,
    pub is_active: bool,
    pub role_name: String,
    pub is_owner: bool,
    pub role: i64,
    pub user: i64
}

#[derive(RustcEncodable, RustcDecodable, Debug)]
pub struct ProjectDetail {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub is_fan: bool,
    pub total_fans: i64,
    pub is_watcher: bool,
    pub total_watchers: i64,
    pub anon_permissions: Vec<String>,
    pub public_permissions: Vec<String>,
    pub my_permissions: Vec<String>,
    pub created_date: String,
    pub modified_date: String,
    pub creation_template: i64,
    pub default_issue_status: i64,
    pub default_issue_type: i64,
    pub default_points: i64,
    pub default_priority: i64,
    pub default_severity: i64,
    pub default_task_status: i64,
    pub default_us_status: i64,
    pub description: String,
    pub i_am_owner: bool,
    pub is_backlog_activated: bool,
    pub is_issues_activated: bool,
    pub is_kanban_activated: bool,
    pub is_wiki_activated: bool,
    pub is_private: bool,
    pub notify_level: Option<i64>,
    pub issue_custom_attributes: Vec<CustomAttribute>,
    pub task_custom_attributes: Vec<CustomAttribute>,
    pub userstory_custom_attributes: Vec<CustomAttribute>,
    pub issue_statuses: Vec<IssueStatus>,
    pub issue_types: Vec<IssueType>,
    pub members: Vec<ProjectMember>,
    pub owner: i64,
    pub points: Vec<Points>,
    pub priorities: Vec<Priority>,
    pub roles: Vec<Role>,
    pub severities: Vec<Severity>,
    pub tags: Vec<String>,
    pub tags_colors: HashMap<String, String>,
    pub task_statuses: Vec<TaskStatus>,
    pub total_milestones: i64,
    pub total_closed_milestones: i64,
    pub total_story_points: f64,
    pub us_statuses: Vec<USStatus>,
    pub videoconferences: Option<String>,
    pub videoconferences_extra_data: Option<String>,
}

#[derive(RustcEncodable, RustcDecodable, Debug)]
pub struct ProjectListItem {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub anon_permissions: Vec<String>,
    pub public_permissions: Vec<String>,
    pub my_permissions: Vec<String>,
    pub created_date: String,
    pub modified_date: String,
    pub creation_template: i64,
    pub default_issue_status: i64,
    pub default_issue_type: i64,
    pub default_points: i64,
    pub default_priority: i64,
    pub default_severity: i64,
    pub default_task_status: i64,
    pub default_us_status: i64,
    pub description: String,
    pub i_am_owner: bool,
    pub is_fan: bool,
    pub total_fans: i64,
    pub is_watcher: bool,
    pub total_watchers: i64,
    pub is_backlog_activated: bool,
    pub is_issues_activated: bool,
    pub is_kanban_activated: bool,
    pub is_wiki_activated: bool,
    pub is_private: bool,
    pub members: Vec<i64>,
    pub owner: i64,
    pub tags: Vec<String>,
    pub tags_colors: HashMap<String, String>,
    pub total_milestones: i64,
    pub total_closed_milestones: i64,
    pub notify_level: Option<i64>,
    pub total_story_points: f64,
    pub videoconferences: Option<String>,
    pub videoconferences_extra_data: Option<String>,
}


