use actix_web::{web, App, HttpResponse, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod agents;
mod metrics;
mod schema;
mod tests;

#[macro_use]
extern crate diesel;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello, api is up.")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // set up database connection pool
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let database_max_connections = std::env::var("DATABASE_MAX_CONNECTIONS").expect("DATABASE_MAX_CONNECTIONS");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .max_size(database_max_connections.parse::<u32>().unwrap())
        .build(manager)
        .expect("Failed to create pool.");

    log::info!("starting HTTP server at 0.0.0.0:8080");

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            // set up DB pool to be used with web::Data<Pool> extractor
            .app_data(web::Data::new(pool.clone()))
            .route("/", web::get().to(index))
            .service(metrics::add_metrics)
            .service(agents::add_agents)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
