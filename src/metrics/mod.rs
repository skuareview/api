use self::model::{InsertableMetric, Metric};
use super::DbPool;
use actix_web::{post, web, Error, HttpResponse};
pub mod model;

/// Inserts new user with name defined in form.
#[post("/metrics")]
pub async fn add_metrics(
    pool: web::Data<DbPool>,
    form: web::Json<InsertableMetric>,
) -> Result<HttpResponse, Error> {
    let metric = web::block(move || {
        let conn = pool.get()?;
        Metric::insert_new_metric(&form, &conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;
    // let bool = email::send_alert_email("service-ses@jenoh.dev".to_string()).await;
    Ok(HttpResponse::Created().json(metric))
}
