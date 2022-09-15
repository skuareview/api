use crate::diesel::RunQueryDsl;
use crate::schema::metrics;
use diesel::prelude::*;
use diesel::{AsChangeset, Queryable};
use serde_derive::{Deserialize, Serialize};

type DbError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Serialize, Deserialize, Queryable, AsChangeset, Insertable)]
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

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
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
    // fn from_metric(metric: Metric) -> InsertableMetric {
    //     InsertableMetric {
    //         load_average_1: metric.load_average_1,
    //         load_average_2: metric.load_average_2,
    //         load_average_3: metric.load_average_3,
    //         memory_used: metric.memory_used,
    //         memory_total: metric.memory_total,
    //         cpu_temp: metric.cpu_temp,
    //         cpu_load: metric.cpu_load,
    //     }
    // }
}

impl Metric {
    pub fn insert_new_metric(
        form: &InsertableMetric,
        conn: &PgConnection,
    ) -> Result<InsertableMetric, DbError> {
        use crate::schema::metrics::dsl::*;

        diesel::insert_into(metrics).values(form).execute(conn)?;

        Ok(form.clone())
    }
}
