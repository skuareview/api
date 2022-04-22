#[cfg(test)]
mod tests {
    use crate::agents;
    use crate::agents::model;
    use actix_web::{test, web, App};
    use diesel::prelude::*;
    use diesel::r2d2::{self, ConnectionManager};

    #[actix_web::test]
    async fn agents_routes() {
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
                .service(agents::add_agents),
        )
        .await;

        // Insert an agent
        let req = test::TestRequest::post()
            .uri("/agents")
            .set_json(&model::AgentName {
                name: "Good name for an agent".to_string(),
            })
            .to_request();

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
