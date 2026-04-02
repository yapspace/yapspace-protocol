use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Login {
	pub login: String,
	pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    pub user_id: String,
    pub token: String,
}
