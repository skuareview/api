use crate::diesel::RunQueryDsl;
use crate::schema::users;
use chrono::{DateTime, Duration, Utc};
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use diesel::prelude::*;
use diesel::{AsChangeset, Queryable};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde_derive::{Deserialize, Serialize};

type DbError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Serialize, Clone, Deserialize, Debug, Queryable, AsChangeset, Insertable)]
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
    pub id: i32,
    pub token: String,
}

impl Login {
    pub fn login(user_login: &Login, conn: &PgConnection) -> Result<LoginResponse, DbError> {
        use crate::schema::users::dsl::*;

        let mut user = users
            .filter(email.eq(user_login.email.clone()))
            .select((id, name, email, password))
            // .first()
            .load::<User>(conn);
        // // .first()
        // .unwrap()
        // .first();
        // if user_id == 10 {
        let mut sha = Sha256::new();
        sha.input_str(&user_login.password);

        // if user.unwrap().password == sha.result_str() {
        let key = std::env::var("SECRET_TOKEN").expect("SECRET_TOKEN");
        // let mut date: DateTime<Utc>;
        let date = Utc::now() + Duration::days(30);

        let my_claims = Claims {
            sub: user_login.email.clone(),
            exp: date.timestamp() as usize,
        };

        let token = encode(
            &Header::default(),
            &my_claims,
            &EncodingKey::from_secret(key.as_bytes()),
        )
        .unwrap();
        // let user_id = user.unwrap()[0].id.clone();
        Ok(LoginResponse {
            id: user.unwrap()[0].id.clone(),
            token: token,
        })
        // }
        // if x.password == sha.result_str()
        // }
        // match users
        //     .filter(email.eq(user_login.email))
        //     .select(id)
        //     .first(conn)
        // {
        //     Some(user) => {}
        //     None => {}
        // }
    }

    // match self.find_user_with_email(user.email.to_string()).unwrap() {
    //     Some(x) => {
    //         let mut sha = Sha256::new();
    //         sha.input_str(user.password.as_str());
    //         if x.password == sha.result_str() {
    //             // JWT
    //             let _config: Config = Config {};
    //             let _var = _config.get_config_with_key("SECRET_KEY");
    //             let key = _var.as_bytes();

    //             let mut _date: DateTime<Utc>;
    //             // Remember Me
    //             if !user.remember_me {
    //                 _date = Utc::now() + Duration::hours(1);
    //             } else {
    //                 _date = Utc::now() + Duration::days(365);
    //             }
    //             let my_claims = Claims {
    //                 sub: user.email,
    //                 exp: _date.timestamp() as usize,
    //             };
    //             let token = encode(
    //                 &Header::default(),
    //                 &my_claims,
    //                 &EncodingKey::from_secret(key),
    //             )
    //                 .unwrap();
    //             Ok(LoginResponse {
    //                 status: true,
    //                 token,
    //                 message: "You have successfully logged in.".to_string(),
    //             })
    //         } else {
    //             Err(Response {
    //                 status: false,
    //                 message: "Check your user informations.".to_string(),
    //             })
    //         }
    //     }
    //     None => Err(Response {
    //         status: false,
    //         message: "Check your user informations.".to_string(),
    //     }),
    // }
}

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
