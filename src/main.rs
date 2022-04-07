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
    load_average_1: String,
    load_average_2: String,
    load_average_3: String,
    memory_used: String,
    memory_total: String,
    cpu_temp: String,
    cpu_load: String,
}

#[post("/metrics", format = "json", data = "<metric>")]
fn metrics(metric: Json<Metric>) -> String {
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
        .mount("/", routes![index, metrics])
        .launch();
}
