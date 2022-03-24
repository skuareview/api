#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use rocket_contrib::json::Json;
use serde::Deserialize;

use rocket_contrib::databases::diesel;

#[derive(Debug, PartialEq, Eq, Deserialize)]
struct Metric {
    loads: (String, String, String),
}

#[post("/metrics", format = "json", data = "<metric>")]
fn helloPost(metric: Json<Metric>) -> String {
    format!("print test {:?}", metric)
}

#[database("postgres")]
struct LogsDbConn(diesel::PgConnection);

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite()
        .attach(LogsDbConn::fairing())
        .mount("/", routes![index, helloPost])
        .launch();
}
