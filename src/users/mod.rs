use self::model::{Login, Register, User};
use super::DbPool;
use actix_web::{post, web, Error, HttpResponse};
pub mod model;

#[post("/register")]
async fn register(
    pool: web::Data<DbPool>,
    user: web::Json<Register>,
) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let conn = pool.get()?;
        Register::register(&user, &conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().json(user))
}

#[post("/login")]
async fn login(pool: web::Data<DbPool>, user: web::Json<Login>) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let conn = pool.get()?;
        Login::login(&user, &conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().json(user))
}
