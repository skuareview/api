#[cfg(test)]
mod tests {
    use crate::roles;
    use crate::roles::model;
    use actix_web::{test, web, App};
    use diesel::prelude::*;
    use diesel::r2d2::{self, ConnectionManager};

    #[actix_web::test]
    async fn roles_routes() {
        // Init
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
                .service(roles::add_role),
        )
        .await;

        // Insert a role
        let req = test::TestRequest::post()
            .uri("/roles")
            .set_json(&model::InsertableRole {
                name: "Admin".to_owned(),
            })
            .to_request();

        let resp: model::InsertableRole = test::call_and_read_body_json(&mut app, req).await;

        assert_eq!(resp.name, "Admin");

        // Get all roles
        // let req = test::TestRequest::get()
        //     .uri(&format!("/roles"))
        //     .to_request();

        // let resp: Vec<model::Role> = test::call_and_read_body_json(&mut app, req).await;

        // assert_eq!(resp.first().name, "Admin");

        // // Delete new user from table
        // use crate::schema::users::dsl::*;
        // diesel::delete(users.filter(id.eq(resp.id)))
        //     .execute(&pool.get().expect("couldn't get db connection from pool"))
        //     .expect("couldn't delete test user from table");
    }
}
