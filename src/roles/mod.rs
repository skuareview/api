use self::model::{InsertableRole, Role};
use super::DbPool;
use actix_web::{get, post, web, Error, HttpResponse};
pub mod model;

/// Inserts new role  with name defined in form.
#[post("/roles")]
pub async fn add_role(
    pool: web::Data<DbPool>,
    form: web::Json<InsertableRole>,
) -> Result<HttpResponse, Error> {
    // use web::block to offload blocking Diesel code without blocking server thread
    let role = web::block(move || {
        let conn = pool.get()?;
        Role::insert_new_role(&form, &conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().json(role))
}

/// Get all roles
#[get("/roles")]
pub async fn get_roles(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    // use web::block to offload blocking Diesel code without blocking server thread
    let roles = web::block(move || {
        let conn = pool.get()?;
        Role::get_all_roles(&conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(roles))
}
