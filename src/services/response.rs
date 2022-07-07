use crate::users::model::{InsertableUser, User};
use serde::{Deserialize, Serialize};

// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct RegisterResponse {
//     pub already_exist: bool,
//     pub user: Option<InsertableUser>,
// }

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
// impl RegisterResponse {
//     pub fn set_register_response(
//         already_exist: bool,
//         user: Option<InsertableUser>,
//     ) -> RegisterResponse {
//         RegisterResponse {
//             already_exist: already_exist,
//             user: user,
//         }
//     }
// }
