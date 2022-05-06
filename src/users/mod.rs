use self::model::{Login, Register, User};
use super::DbPool;
use crate::middlewares::auth::AuthorizationService;
use actix_web::{get, post, web, Error, HttpRequest, HttpResponse};
pub mod model;

#[post("/register")]
pub async fn register(
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
pub async fn login(pool: web::Data<DbPool>, user: web::Json<Login>) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let conn = pool.get()?;
        Login::login(&user, &conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().json(user))
}

#[get("/user_informations")]
pub async fn user_informations(
    _req: HttpRequest,
    pool: web::Data<DbPool>,
    _: AuthorizationService,
) -> Result<HttpResponse, Error> {
    let token = User::get_uid_from_request(&_req);
    let user = web::block(move || {
        let conn = pool.get()?;
        User::get_user_informations(&token, &conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().json(user))
}
