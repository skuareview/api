#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use rocket_contrib::json::Json;
use serde::Deserialize;

use rocket_contrib::databases::diesel;

mod db;

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

fn main() {
    let mut rocket = rocket::ignite().manage(db::init_pool());
    rocket.launch();
}
