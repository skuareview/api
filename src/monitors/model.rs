use crate::diesel::RunQueryDsl;
use crate::schema::monitors;
use diesel::prelude::*;
use diesel::{AsChangeset, Queryable};
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

type DbError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Serialize, Deserialize, Queryable, AsChangeset, Insertable)]
#[table_name = "monitors"]
pub struct Monitor {
    pub id: i32,
    pub id_agent: Option<i32>,
    pub id_lambda: Option<i32>,
    pub id_user: Uuid,
    pub id_organization: Option<i32>,
    pub name: String,
    pub aws_eventbridge_region: String,
    pub aws_eventbridge_name: String,
    pub aws_eventbridge_description: String,
    pub aws_eventbridge_event_bus_name: String,
    pub aws_eventbridge_schedule_expression: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, Queryable)]
#[table_name = "monitors"]
pub struct InsertableMonitor {
    pub id_agent: Option<i32>,
    pub id_lambda: Option<i32>,
    pub id_user: Uuid,
    pub id_organization: Option<i32>,
    pub name: String,
    pub aws_eventbridge_region: String,
    pub aws_eventbridge_name: String,
    pub aws_eventbridge_description: String,
    pub aws_eventbridge_event_bus_name: String,
    pub aws_eventbridge_schedule_expression: String,
}

#[derive(Debug, Clone, Serialize, Queryable, Deserialize, Insertable)]
#[table_name = "monitors"]
pub struct FormMonitor {
    pub name: String,
    pub id_organization: Option<i32>,
    pub id_agent: Option<i32>,
}

impl Monitor {
    pub fn insert_new_monitor(
        token: &str,
        form: &FormMonitor,
        conn: &PgConnection,
    ) -> Result<InsertableMonitor, DbError> {
        use crate::schema::monitors::dsl::*;
        let uid = crate::users::model::User::get_uid_from_token(token, conn);
        let monitor = InsertableMonitor {
            name: form.name.clone(),
            aws_eventbridge_region: "wip".to_owned(),
            aws_eventbridge_name: "wip".to_owned(),
            aws_eventbridge_description: "wip".to_owned(),
            aws_eventbridge_event_bus_name: "wip".to_owned(),
            aws_eventbridge_schedule_expression: "wip".to_owned(),
            id_agent: form.id_agent,
            id_lambda: None,
            id_user: uid.unwrap(),
            id_organization: form.id_organization,
        };
        diesel::insert_into(monitors)
            .values(&monitor)
            .execute(conn)?;

        Ok(monitor)
    }
    // pub fn get_all_monitors_of_user(
    //     token: &str,
    //     conn: &PgConnection,
    // ) -> Result<Vec<Monitor>, DbError> {
    //     let uid = crate::users::model::User::get_uid_from_token(token, conn);
    //     use crate::schema::monitors::dsl::*;

    //     let all_monitors = monitors
    //         .filter(id_user.eq(uid.unwrap()))
    //         .load::<Monitor>(conn)
    //         .unwrap();
    //     Ok(all_monitors)
    // }
}
