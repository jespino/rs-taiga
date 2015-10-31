use time::Tm;

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
// Common
//

#[derive(RustcEncodable, RustcDecodable, Debug)]
pub struct CustomAttribute {
	pub id: i64,
	pub name: String,
	pub description: String,
	pub order: i64,
	pub project: i64
}

#[derive(RustcEncodable, RustcDecodable, Debug)]
pub struct IssueStatus {
	pub id: i64,
	pub color: String,
	pub is_closed: bool,
	pub name: String,
	pub order: i64,
	pub project: i64,
	pub slug: String
}

#[derive(RustcEncodable, RustcDecodable, Debug)]
pub struct IssueType {
	pub id: i64,
	pub color: String,
	pub name: String,
	pub order: i64,
	pub project: i64,
}

#[derive(RustcEncodable, RustcDecodable, Debug)]
pub struct Membership {
	pub id: i64,
	pub color: String,
	pub created_at: String,
	pub email: String,
	pub full_name: String,
	pub invitation_extra_text: String,
	pub invited_by: Option<i64>,
	pub is_owner: bool,
	pub photo: String,
	pub project: i64,
	pub role: i64,
	pub role_name: String,
	pub token: Option<String>,
	pub user: i64
}

#[derive(RustcEncodable, RustcDecodable, Debug)]
pub struct Points {
	pub id: i64,
	pub name: String,
	pub order: i64,
	pub project: i64,
	pub value: Option<i64>
}

#[derive(RustcEncodable, RustcDecodable, Debug)]
pub struct Priority {
    pub id: i64,
    pub color: String,
    pub name: String,
    pub order: i64,
    pub project: i64,
}

#[derive(RustcEncodable, RustcDecodable, Debug)]
pub struct Role {
	pub id: i64,
	pub computable: bool,
	pub name: String,
	pub order: i64,
	pub slug: String
}

#[derive(RustcEncodable, RustcDecodable, Debug)]
pub struct Severity {
	pub id: i64,
	pub color: String,
	pub name: String,
	pub order: i64,
	pub project: i64,
}

#[derive(RustcEncodable, RustcDecodable, Debug)]
pub struct TagColor {
	name: String,
	color: String,
}

#[derive(RustcEncodable, RustcDecodable, Debug)]
pub struct TaskStatus {
    pub id: i64,
    pub color: String,
    pub is_closed: bool,
    pub name: String,
    pub order: i64,
    pub project: i64,
    pub slug: String
}

#[derive(RustcEncodable, RustcDecodable, Debug)]
pub struct USStatus {
	pub id: i64,
	pub color: String,
	pub is_closed: bool,
	pub name: String,
	pub order: i64,
	pub project: i64,
	pub slug: String,
	pub wip_limit: Option<i64>
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

#[derive(RustcEncodable, RustcDecodable, Debug)]
pub struct ProjectUser {
	pub id: i64,
	pub username: String,
	pub full_name: String,
	pub full_name_display: String,
	pub email: String,
	pub github_id: Option<String>,
	pub color: String,
	pub bio: String,
	pub lang: String,
	pub timezone: String,
	pub is_active: bool,
	pub photo: String,
	pub big_photo: String
}

#[derive(RustcEncodable, RustcDecodable, Debug)]
pub struct Project {
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
    pub is_backlog_activated: bool,
    pub is_issues_activated: bool,
    pub is_kanban_activated: bool,
    pub is_private: bool,
    pub is_wiki_activated: bool,
	pub issue_custom_attributes: Vec<CustomAttribute>,
	pub task_custom_attributes: Vec<CustomAttribute>,
	pub userstory_custom_attributes: Vec<CustomAttribute>,
	pub issue_statuses: Vec<IssueStatus>,
	pub issue_types: Vec<IssueType>,
	pub members: Vec<i64>,
	pub memberships: Vec<Membership>,
	pub owner: i64,
	pub points: Vec<Points>,
    pub priorities: Vec<Priority>,
    pub roles: Vec<Role>,
	pub severities: Vec<Severity>,
	pub stars: i64,
	pub tags: Vec<String>,
	pub tags_colors: Vec<TagColor>,
	pub task_statuses: Vec<TaskStatus>,
	pub total_milestones: i64,
	pub total_story_points: f64,
	pub us_statuses: Vec<USStatus>,
	pub users: Vec<ProjectUser>,
	pub videoconferences: Option<String>,
	pub videoconferences_salt: Option<String>,
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

#[derive(RustcEncodable, RustcDecodable, Debug)]
pub struct UserStory {
    pub id: i64,
    pub subject: String,
}
