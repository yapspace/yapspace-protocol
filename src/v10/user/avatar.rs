use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Avatar {
    pub id: i64,
    pub user_id: i64,
}
