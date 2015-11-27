#[derive(Serialize, Deserialize, Debug)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
    #[serde(rename="type")]
    pub login_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginResponse {
    pub id: i64,
    pub color: String,
    pub timezone: String,
    pub auth_token: String,
    pub theme: String,
    pub big_photo: String,
    pub photo: String,
    pub username: String,
    pub full_name: String,
    pub full_name_display: String,
    pub bio: String,
    pub email: String,
    pub lang: String,
    pub is_active: bool
}
