use crate::diesel::RunQueryDsl;
use crate::schema::users;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use diesel::prelude::*;
use diesel::{AsChangeset, Queryable};
use serde_derive::{Deserialize, Serialize};

type DbError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Serialize, Deserialize, Debug, Queryable, AsChangeset, Insertable)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Clone, Deserialize, Debug, Insertable)]
#[table_name = "users"]
pub struct InsertableUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Login {
    pub email: String,
    pub password: String,
    // #[serde(default)]
    // pub remember_me: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Register {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginResponse {
    pub message: String,
    pub status: bool,
    pub token: String,
}

// #[derive(Serialize, Deserialize, Debug)]
// pub struct Response {
//     pub message: String,
//     pub status: bool,
// }

impl Register {
    fn to_insertable_user(user_register: &Register, password_crypt: String) -> InsertableUser {
        InsertableUser {
            name: user_register.name.to_owned(),
            email: user_register.email.to_owned(),
            password: password_crypt,
        }
    }
    pub fn register(
        user_register: &Register,
        conn: &PgConnection,
    ) -> Result<InsertableUser, DbError> {
        let mut sha = Sha256::new();
        sha.input_str(user_register.password.as_str());
        let hash_pw = sha.result_str();

        use crate::schema::users::dsl::*;
        let new_user = Register::to_insertable_user(user_register, hash_pw);
        diesel::insert_into(users)
            .values(new_user.clone())
            .execute(conn)?;

        Ok(new_user)
    }
}
