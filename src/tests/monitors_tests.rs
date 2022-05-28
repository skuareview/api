#[cfg(test)]
mod tests {
    use crate::monitors;
    use crate::monitors::model;
    use actix_web::{test, web, App};
    use diesel::prelude::*;
    use diesel::r2d2::{self, ConnectionManager};

    #[actix_web::test]
    async fn monitors_routes() {
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
                .service(monitors::add_monitor),
        )
        .await;

        use crate::schema::users::dsl::*;
        let new_user = crate::users::model::InsertableUser {
            name: "testMonitors".to_owned(),
            email: "testMonitors@gmail.com".to_owned(),
            password: "password_crypt".to_owned(),
            id_role: 1,
        };
        let testUser = diesel::insert_into(users)
            .values(new_user.clone())
            .execute(&conn);

        let user = crate::users::model::User::find_user_with_email(
            "testMonitors@gmail.com".to_owned(),
            &conn,
        );

        // Retreive token
        let token = crate::users::model::User::find_token(user.unwrap().email.clone());
        // Insert a monitor
        let req = test::TestRequest::post()
            .uri("/monitors")
            .insert_header((
                actix_web::http::header::AUTHORIZATION,
                "Bearer ".to_owned() + &token,
            ))
            .set_json(&model::FormMonitor {
                name: "my best monitor".to_owned(),
                id_organization: None,
                id_agent: None,
            })
            .to_request();

        let resp: model::InsertableMonitor = test::call_and_read_body_json(&mut app, req).await;

        assert_eq!(resp.name, "my best monitor");

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
