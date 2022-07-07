#[cfg(test)]
mod tests {
    use crate::services::response;
    use crate::users as users_crate;
    use crate::users::model;
    use actix_web::{test, web, App};
    use diesel::prelude::*;
    use diesel::r2d2::{self, ConnectionManager};
    use rand::Rng;
    use uuid::Uuid;

    #[actix_web::test]
    async fn register() {
        /*
         * Arrange
         */
        std::env::set_var("RUST_LOG", "actix_web=debug");
        dotenv::dotenv().ok();

        let connspec = std::env::var("DATABASE_URL_TEST").expect("DATABASE_URL_TEST");
        let manager = ConnectionManager::<PgConnection>::new(connspec);
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");

        let mut app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .service(users_crate::register),
        )
        .await;

        /*
         * Act
         */
        let mut rng = rand::thread_rng();
        let random: String = rng.gen::<i32>().to_string();

        let req = test::TestRequest::post()
            .uri("/register")
            .set_json(&model::Register {
                name: "Phantom98".to_owned(),
                email: "phantom@gmail.com".to_owned() + &random,
                password: "phantom".to_owned(),
            })
            .to_request();

        let resp: response::CustomResponse = test::call_and_read_body_json(&mut app, req).await;

        /*
         * Assert
         */
        assert_eq!(resp.status, true);
    }

    #[actix_web::test]
    async fn login() {
        /*
         * Arrange
         */
        std::env::set_var("RUST_LOG", "actix_web=debug");
        dotenv::dotenv().ok();

        let connspec = std::env::var("DATABASE_URL_TEST").expect("DATABASE_URL_TEST");
        let manager = ConnectionManager::<PgConnection>::new(connspec);
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        let conn = pool.get().unwrap();

        let mut app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .service(users_crate::login),
        )
        .await;

        let mut rng = rand::thread_rng();
        let random: String = rng.gen::<i32>().to_string();
        use crate::schema::users::dsl::*;
        let uuid = Uuid::new_v4();

        let new_user = crate::users::model::InsertableUser {
            id: uuid,
            name: "seed_user_name".to_owned() + &random,
            email: "seed_user_email@gmail.com".to_owned() + &random,
            password: model::User::hash_pw("seed_user_password".to_owned() + &random),
            id_role: 1,
        };
        diesel::insert_into(users)
            .values(new_user.clone())
            .execute(&conn)
            .unwrap();

        /*
         * Act
         */
        let req = test::TestRequest::post()
            .uri("/login")
            .set_json(&model::Login {
                email: "seed_user_email@gmail.com".to_owned() + &random,
                password: "seed_user_password".to_owned() + &random,
            })
            .to_request();

        let resp: response::LoginResponse = test::call_and_read_body_json(&mut app, req).await;

        /*
         * Assert
         */
        assert_eq!(resp.status, true);
    }

    #[actix_web::test]
    async fn user_informations() {
        /*
         * Arrange
         */
        std::env::set_var("RUST_LOG", "actix_web=debug");
        dotenv::dotenv().ok();

        let connspec = std::env::var("DATABASE_URL_TEST").expect("DATABASE_URL_TEST");
        let manager = ConnectionManager::<PgConnection>::new(connspec);
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        let conn = pool.get().unwrap();

        let mut app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .service(users_crate::user_informations),
        )
        .await;

        let mut rng = rand::thread_rng();
        let random: String = rng.gen::<i32>().to_string();
        use crate::schema::users::dsl::*;
        let uuid = Uuid::new_v4();

        let new_user = crate::users::model::InsertableUser {
            id: uuid,
            name: "seed_user_name".to_owned() + &random,
            email: "seed_user_email@gmail.com".to_owned() + &random,
            password: model::User::hash_pw("seed_user_password".to_owned() + &random),
            id_role: 1,
        };
        diesel::insert_into(users)
            .values(new_user.clone())
            .execute(&conn)
            .unwrap();

        /*
         * Act
         */
        let token = crate::users::model::User::find_token(new_user.email.clone());

        let req = test::TestRequest::get()
            .uri("/user_informations")
            .insert_header((
                actix_web::http::header::AUTHORIZATION,
                "Bearer ".to_owned() + &token,
            ))
            .to_request();

        let resp: response::UserResponse = test::call_and_read_body_json(&mut app, req).await;

        /*
         * Assert
         */
        assert_eq!(resp.status, true);
    }
}
