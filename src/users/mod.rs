use self::model::{Login, Register, User};
use super::DbPool;
use actix_web::{get, post, web, Error, HttpRequest, HttpResponse};
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

// #[get("/user_informations")]
// async fn user_informations(
//     _req: HttpRequest,
//     pool: web::Data<DbPool>,
// ) -> Result<HttpResponse, Error> {
//     let _auth = _req.headers().get("Authorization");
//     let _split: Vec<&str> = _auth.unwrap().to_str().unwrap().split("Bearer").collect();
//     let token = _split[1].trim();
//     let user = web::block(move || {
//         let conn = pool.get()?;
//         User::get_user_informations(token, &conn)
//     })
//     .await?
//     .map_err(actix_web::error::ErrorInternalServerError)?;

//     Ok(HttpResponse::Created().json(user))
// }
