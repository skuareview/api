#[cfg(test)]
mod tests {
    use crate::metrics;
    use crate::metrics::model;
    use actix_web::{test, web, App};
    use diesel::prelude::*;
    use diesel::r2d2::{self, ConnectionManager};

    #[actix_web::test]
    async fn metrics_routes() {
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
                .service(metrics::add_metrics),
        )
        .await;

        // Insert a metric
        let req = test::TestRequest::post()
            .uri("/metrics")
            .set_json(&model::InsertableMetric {
                load_average_1: Some("0.10".to_owned()),
                load_average_2: Some("1.32".to_owned()),
                load_average_3: Some("5.90".to_owned()),
                memory_used: Some("780".to_owned()),
                memory_total: Some("2048".to_owned()),
                cpu_temp: Some("40.12".to_owned()),
                cpu_load: Some("32.20".to_owned()),
            })
            .to_request();

        let resp: model::InsertableMetric = test::call_and_read_body_json(&mut app, req).await;

        assert_eq!(resp.load_average_1.unwrap(), "0.10");

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
