use self::model::{FormMonitor, InsertableMonitor, Monitor};
use super::DbPool;
use crate::middlewares::auth::AuthorizationService;
use actix_web::{get, post, web, Error, HttpRequest, HttpResponse};
pub mod model;

/// Inserts new monitor  with name, id_agent, id_organization defined in form.
#[post("/monitors")]
pub async fn add_monitor(
    pool: web::Data<DbPool>,
    form: web::Json<FormMonitor>,
    _req: HttpRequest,
    _: AuthorizationService,
) -> Result<HttpResponse, Error> {
    let token = crate::users::model::User::get_token_from_request(&_req);

    let monitor = web::block(move || {
        let conn = pool.get()?;
        Monitor::insert_new_monitor(&token, &form, &conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().json(monitor))
}

// Get all monitors of the user
// #[get("/monitors")]
// pub async fn get_all_monitors_of_user(
//     pool: web::Data<DbPool>,
//     _req: HttpRequest,
//     _: AuthorizationService,
// ) -> Result<HttpResponse, Error> {
//     let token = crate::users::model::User::get_token_from_request(&_req);

//     let monitors = web::block(move || {
//         let conn = pool.get()?;
//         Monitor::get_all_monitors_of_user(&token, &conn)
//     })
//     .await?
//     .map_err(actix_web::error::ErrorInternalServerError)?;

//     Ok(HttpResponse::Ok().json(monitors))
// }
