pub mod model;

use self::model::Metric;
use rocket::{self, http::Status};
use rocket_contrib::json::{Json, JsonValue};

use crate::db;

// #[post("/", data = "<post>")]
// fn create(_key: ApiKey, post: Json<Post>, connection: db::DbConn) -> Result<Json<Post>, Status> {
//     Post::create(post.into_inner(), &connection)
//         .map(Json)
//         .map_err(|_| Status::InternalServerError)
// }

// #[post("/", data = "<post>", rank = 2)]
// fn create_error(post: Json<Post>) -> Json<JsonValue> {
//     Json(json!(
//         {
//             "success":false,
//             "message": "Not authorized"
//         }
//     ))
// }

#[get("/")]
fn read(connection: db::DbConn) -> Result<Json<JsonValue>, Status> {
    Metric::get_all(&connection)
        .map(|item| Json(json!(item)))
        .map_err(|_| Status::NotFound)
}

pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket.mount("/metrics", routes![read])
}
