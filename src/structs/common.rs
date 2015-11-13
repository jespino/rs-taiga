pub struct Taiga {
    pub url: String,
    pub token: Option<String>,
}

#[derive(Debug)]
pub struct APIError {
    pub message: String
}

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
    pub value: Option<f64>
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

#[derive(PartialEq, Debug)]
pub enum ObjectType {
    Project,
    UserStory,
    Issue,
    Task,
    WikiPage,
    Attachment,
}

pub struct DeleteProxy<'a> {
    pub taiga_client: &'a Taiga,
    pub object_type: ObjectType,
    pub object_id: i64,
}
