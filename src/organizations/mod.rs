use self::model::{InsertableOrganisation, Organization};
use super::DbPool;
use crate::middlewares::auth::AuthorizationService;
use actix_web::{get, post, web, Error, HttpRequest, HttpResponse};
pub mod model;

/// Insert new organization.
#[post("/organizations")]
pub async fn add_organization(
    pool: web::Data<DbPool>,
    form: web::Json<InsertableOrganisation>,
    _req: HttpRequest,
    _: AuthorizationService,
) -> Result<HttpResponse, Error> {
    let token = crate::users::model::User::get_token_from_request(&_req);
    let monitor = web::block(move || {
        let conn = pool.get()?;
        Organization::insert_new_organization(&token, &form, &conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().json(monitor))
}

// /// Get all monitors of the user
// #[get("/monitors")]
// pub async fn get_all_monitors_of_user(
//     pool: web::Data<DbPool>,
//     _req: HttpRequest,
//     _: AuthorizationService,
// ) -> Result<HttpResponse, Error> {
//     let token = crate::users::model::User::get_token_from_request(&_req);

//     // use web::block to offload blocking Diesel code without blocking server thread
//     let monitors = web::block(move || {
//         let conn = pool.get()?;
//         Monitor::get_all_monitors_of_user(&token, &conn)
//     })
//     .await?
//     .map_err(actix_web::error::ErrorInternalServerError)?;

//     Ok(HttpResponse::Created().json(monitors))
// }
