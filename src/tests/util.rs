use crate::services::response;
use crate::users as users_crate;
use crate::users::model;
use actix_web::{test, web, App};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use rand::Rng;
use uuid::Uuid;

pub fn insert_user(salt: String, conn: &PgConnection) -> String {
    use crate::schema::users::dsl::*;
    let uuid = Uuid::new_v4();

    let new_user = crate::users::model::InsertableUser {
        id: uuid,
        name: "seed_user_name".to_owned() + &salt,
        email: "seed_user_email@gmail.com".to_owned() + &salt,
        password: model::User::hash_pw("seed_user_password".to_owned() + &salt),
        id_role: 1,
    };
    diesel::insert_into(users)
        .values(new_user.clone())
        .execute(conn)
        .unwrap();
    return crate::users::model::User::find_token(new_user.email.clone());
}
