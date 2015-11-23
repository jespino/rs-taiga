#[derive(Serialize, Deserialize, Debug)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
    #[serde(rename="type")]
    pub login_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginResponse {
    pub auth_token: String
}
