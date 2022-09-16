use actix_web::{post, web, Result, HttpResponse, Error};
use uuid::Uuid;

use crate::monitors::model::*;
mod model;
mod util;


#[post("/monitors")]
async fn add_monitor(monitor: web::Json<Monitor>) -> Result<HttpResponse, Error> {
    Monitor::write_new_monitor(monitor)?;
    //Monitor::insert_new_monitor(monitor)?;
    Ok(HttpResponse::Ok().body("Service created"))
}

