#[cfg(test)]
mod tests {
    use crate::agents;
    use crate::agents::model;
    use crate::users;
    use actix_web::{test, web, App};
    use chrono::{Duration, Utc};
    use diesel::prelude::*;
    use diesel::r2d2::{self, ConnectionManager};
    use jsonwebtoken::{encode, EncodingKey, Header};
    use crate::services::response;

    #[actix_web::test]

    async fn agents_routes() {
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
                .service(agents::add_agents),
        )
        .await;

        // // Find user
        // let user: Option<users::model::User> =
        //     users::model::User::find_user_with_name("Phantom98".to_owned(), &conn);

        // let mut app = test::init_service(
        //     App::new()
        //         .app_data(web::Data::new(pool.clone()))
        //         .service(creausers::register),
        // )
        // .await;

        // // Register a user
        // let req = test::TestRequest::post()
        //     .uri("/register")
        //     .set_json(&users::model::Register {
        //         name: "testAgents".to_owned(),
        //         email: "testAgents@gmail.com".to_owned(),
        //         password: "testAgents".to_owned(),
        //     })
        //     .to_request();

        // let resp: response::CustomResponse = test::call_and_read_body_json(&mut app, req).await;

        use crate::schema::users::dsl::*;
        let new_user = crate::users::model::InsertableUser {
            name: "testAgents".to_owned(),
            email: "testAgents@gmail.com".to_owned(),
            password: "password_crypt".to_owned(),
            id_role: 1,
        };
        let testUser = diesel::insert_into(users)
            .values(new_user.clone())
            .execute(&conn);

        let user = crate::users::model::User::find_user_with_email(
            "testAgents@gmail.com".to_owned(),
            &conn,
        );

        println!("{:?}", user);
        // Retreive token
        let token = crate::users::model::User::find_token(user.unwrap().email.clone());
        let req = test::TestRequest::post()
            .uri("/agents")
            .insert_header((
                actix_web::http::header::AUTHORIZATION,
                "Bearer ".to_owned() + &token,
            ))
            .set_json(&model::AgentName {
                name: "Good name for an agent".to_string(),
            })
            .to_request();
        println!("{:?}", req);
        let resp: model::InsertableAgent = test::call_and_read_body_json(&mut app, req).await;

        assert_eq!(resp.name, "Good name for an agent");

        // // Get a user
        // let req = test::TestRequest::get()
        //     .uri(&format!("/user/{}", resp.id))
        //     .to_request();

        // let resp: models::User = test::call_and_read_body_json(&mut app, req).await;

        // assert_eq!(resp.name, "Test user");

        // // Delete new user from table
        // use crate::schema::users::dsl::*;
        // diesel::delete(users.filter(id.eq(resp.id)))
        //     .execute(&pool.get().expect("couldn't get db connection from pool"))
        //     .expect("couldn't delete test user from table");
    }
}
