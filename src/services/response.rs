use crate::users::model::InsertableUser;
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
    pub id: Option<i32>,
    pub token: Option<String>,
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
