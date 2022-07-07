use crate::diesel::RunQueryDsl;
use crate::roles::model::Role;
use crate::schema::users;
use crate::services::response::{CustomResponse, LoginResponse, UserResponse};
use actix_web::{get, post, web, Error, HttpRequest, HttpResponse};
use chrono::{DateTime, Duration, Utc};
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use diesel::prelude::*;
use diesel::{AsChangeset, Queryable};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

type DbError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Serialize, Clone, Deserialize, Debug, Queryable, AsChangeset)]
#[table_name = "users"]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Clone, Deserialize, Debug, Insertable)]
#[table_name = "users"]
pub struct InsertableUser {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub id_role: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Login {
    pub email: String,
    pub password: String,
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

impl User {
    pub fn find_user_with_email(user_email: String, conn: &PgConnection) -> Option<User> {
        use crate::schema::users::dsl::*;
        let user = users
            .filter(email.eq(user_email))
            .select((id, name, email, password))
            .load::<User>(conn)
            .unwrap();
        if user.len() > 0 {
            return Some(user[0].clone());
        } else {
            return None;
        }
    }
    pub fn find_token(user_email: String) -> String {
        let key = std::env::var("SECRET_TOKEN").expect("SECRET_TOKEN");

        let date = Utc::now() + Duration::days(30);
        let my_claims = Claims {
            sub: user_email.clone(),
            exp: date.timestamp() as usize,
        };
        let token = encode(
            &Header::default(),
            &my_claims,
            &EncodingKey::from_secret(key.as_bytes()),
        )
        .unwrap();
        return token;
    }
    pub fn find_user_with_name(user_name: String, conn: &PgConnection) -> Option<User> {
        use crate::schema::users::dsl::*;
        let user = users
            .filter(name.eq(user_name))
            .select((id, name, email, password))
            .load::<User>(conn)
            .unwrap();
        if user.len() > 0 {
            return Some(user[0].clone());
        } else {
            return None;
        }
    }
    pub fn get_token_from_request(request: &HttpRequest) -> String {
        let _auth = request.headers().get("Authorization");
        let _split: Vec<&str> = _auth.unwrap().to_str().unwrap().split("Bearer").collect();
        let token = _split[1].trim();
        return token.to_string();
    }

    pub fn get_uid_from_token(token: &str, conn: &PgConnection) -> Option<uuid::Uuid> {
        let key = std::env::var("SECRET_TOKEN").expect("SECRET_TOKEN");
        let _decode = decode::<Claims>(
            token,
            &DecodingKey::from_secret(key.as_bytes()),
            &Validation::new(Algorithm::HS256),
        );

        match _decode {
            Ok(decoded) => {
                match User::find_user_with_email(
                    (decoded.claims.sub.to_string()).parse().unwrap(),
                    conn,
                ) {
                    Some(user) => return Some(user.id),
                    None => return None,
                }
            }
            Err(_) => return None,
        }
    }

    pub fn get_user_informations(
        token: &str,
        conn: &PgConnection,
    ) -> Result<UserResponse, DbError> {
        let key = std::env::var("SECRET_TOKEN").expect("SECRET_TOKEN");
        let _decode = decode::<Claims>(
            token,
            &DecodingKey::from_secret(key.as_bytes()),
            &Validation::new(Algorithm::HS256),
        );
        match _decode {
            Ok(decoded) => {
                match User::find_user_with_email(
                    (decoded.claims.sub.to_string()).parse().unwrap(),
                    conn,
                ) {
                    Some(user) => Ok(UserResponse {
                        status: true,
                        user: Some(user),
                    }),
                    None => Ok(UserResponse {
                        status: false,
                        user: None,
                    }),
                }
            }
            Err(_) => Ok(UserResponse {
                status: false,
                user: None,
            }),
        }
    }
    pub fn hash_pw(password: String) -> String {
        let mut sha = Sha256::new();
        sha.input_str(&password);
        return sha.result_str();
    }
}

impl Login {
    pub fn login(user_login: &Login, conn: &PgConnection) -> Result<LoginResponse, DbError> {
        // get the potential user with email
        let user = User::find_user_with_email(user_login.email.clone(), conn);
        match user {
            Some(_) => {
                let login_user = user.unwrap().clone();
                let mut sha = Sha256::new();
                sha.input_str(&user_login.password);
                if login_user.password == sha.result_str() {
                    let token = User::find_token(user_login.email.clone());
                    return Ok(LoginResponse {
                        status: true,
                        id: Some(login_user.id),
                        token: Some(token),
                    });
                } else {
                    return Ok(LoginResponse {
                        status: false,
                        id: None,
                        token: None,
                    });
                }
            }
            None => {
                return Ok(LoginResponse {
                    status: true,
                    id: None,
                    token: None,
                });
            }
        }
    }
}

impl Register {
    fn to_insertable_user(user_register: &Register, password_crypt: String) -> InsertableUser {
        let id = Uuid::new_v4();
        InsertableUser {
            id: id,
            name: user_register.name.to_owned(),
            email: user_register.email.to_owned(),
            password: password_crypt,
            id_role: Role::USER,
        }
    }
    pub fn register(
        user_register: &Register,
        conn: &PgConnection,
    ) -> Result<CustomResponse, DbError> {
        // Check if user already exist
        let _exist = User::find_user_with_email(user_register.email.clone(), conn);
        match _exist {
            Some(_) => {
                return Ok(CustomResponse {
                    status: false,
                    message: "exist".to_string(),
                });
            }
            None => {
                // Register user
                let hash_pw: String = User::hash_pw(user_register.password.clone());
                use crate::schema::users::dsl::*;
                let new_user = Register::to_insertable_user(user_register, hash_pw);
                diesel::insert_into(users)
                    .values(new_user.clone())
                    .execute(conn)?;

                return Ok(CustomResponse {
                    status: true,
                    message: "Created".to_string(),
                });
            }
        }
    }
}
