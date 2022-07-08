#[cfg(test)]
mod tests {
    use crate::monitors;
    use crate::monitors::model;
    use crate::tests::util;
    use actix_web::{test, web, App};
    use diesel::prelude::*;
    use diesel::r2d2::{self, ConnectionManager};
    use rand::Rng;

    #[actix_web::test]
    async fn post() {
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
                .service(monitors::add_monitor),
        )
        .await;

        let mut rng = rand::thread_rng();
        let random: String = rng.gen::<i32>().to_string();
        let token: String = util::insert_user(random.clone(), &conn);

        /*
         * Act
         */
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

        /*
         * Assert
         */
        assert_eq!(resp.name, "my best monitor");
    }
}
