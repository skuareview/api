use crate::schema::metrics;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Queryable, AsChangeset)]
#[table_name = "metrics"]
pub struct Metric {
    pub id: i32,
    pub load_average_1: Option<String>,
    pub load_average_2: Option<String>,
    pub load_average_3: Option<String>,
    pub memory_used: Option<String>,
    pub memory_total: Option<String>,
    pub cpu_temp: Option<String>,
    pub cpu_load: Option<String>,
}

#[derive(Insertable)]
#[table_name = "metrics"]
pub struct InsertableMetric {
    pub load_average_1: Option<String>,
    pub load_average_2: Option<String>,
    pub load_average_3: Option<String>,
    pub memory_used: Option<String>,
    pub memory_total: Option<String>,
    pub cpu_temp: Option<String>,
    pub cpu_load: Option<String>,
}

impl InsertableMetric {
    fn from_metric(metric: Metric) -> InsertableMetric {
        InsertableMetric {
            load_average_1: metric.load_average_1,
            load_average_2: metric.load_average_2,
            load_average_3: metric.load_average_3,
            memory_used: metric.memory_used,
            memory_total: metric.memory_total,
            cpu_temp: metric.cpu_temp,
            cpu_load: metric.cpu_load,
        }
    }
}

impl Metric {
    pub fn create(metric: Metric, connection: &PgConnection) -> QueryResult<Metric> {
        diesel::insert_into(metrics::table)
            .values(&InsertableMetric::from_metric(metric))
            .execute(connection)?;
        metrics::table.order(metrics::id.desc()).first(connection)
    }

    pub fn get_all(connection: &PgConnection) -> QueryResult<Vec<Metric>> {
        metrics::table.order(metrics::id).load::<Metric>(connection)
    }
}
