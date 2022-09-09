use crate::users::model::User;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CustomResponse {
    pub status: bool,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginResponse {
    pub status: bool,
    pub id: Option<uuid::Uuid>,
    pub token: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UserResponse {
    pub status: bool,
    pub user: Option<User>,
}
