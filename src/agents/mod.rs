use self::model::{Agent, AgentName};
use super::DbPool;
use actix_web::{post, web, Error, HttpResponse};

pub mod model;

/// Inserts new user with name defined in form.
#[post("/agents")]
pub async fn add_agents(
    pool: web::Data<DbPool>,
    agent_name: web::Json<AgentName>,
) -> Result<HttpResponse, Error> {
    // use web::block to offload blocking Diesel code without blocking server thread
    let agent = web::block(move || {
        let conn = pool.get()?;
        Agent::insert_new_agent(&agent_name.name.to_string(), &conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().json(agent))
}
