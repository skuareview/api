use crate::diesel::RunQueryDsl;
use crate::schema::organizations;
use crate::schema::organizationsusers;
use diesel::prelude::*;
use diesel::{AsChangeset, Queryable};
use serde_derive::{Deserialize, Serialize};

type DbError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Serialize, Deserialize, Queryable, AsChangeset, Insertable)]
#[table_name = "organizations"]
pub struct Organization {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, Queryable)]
#[table_name = "organizations"]
pub struct InsertableOrganisation {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, Queryable)]
#[table_name = "organizationsusers"]
pub struct InsertableOrganisationUser {
    pub id_organization: Option<i32>,
    pub id_user: Option<i32>,
}

// #[derive(Debug, Clone, Serialize, Queryable, Deserialize, Insertable)]
// #[table_name = "monitors"]
// pub struct FormMonitor {
//     pub name: String,
//     pub id_organization: Option<i32>,
//     pub id_agent: Option<i32>,
// }

impl Organization {
    pub fn insert_new_organization(
        token: &str,
        form: &InsertableOrganisation,
        conn: &PgConnection,
    ) -> Result<InsertableOrganisation, DbError> {
        use crate::schema::organizations::dsl::*;
        use crate::schema::organizationsusers::dsl::*;
        // let monitor = InsertableMonitor {
        //     name: form.name.clone(),
        //     aws_eventbridge_region: "wip".to_owned(),
        //     aws_eventbridge_name: "wip".to_owned(),
        //     aws_eventbridge_description: "wip".to_owned(),
        //     aws_eventbridge_event_bus_name: "wip".to_owned(),
        //     aws_eventbridge_schedule_expression: "wip".to_owned(),
        //     id_agent: form.id_agent,
        //     id_lambda: None,
        //     id_organization: form.id_organization,
        // };
        let su = diesel::insert_into(organizations)
            .values(form)
            .execute(conn)?;
        println!("{:?}", su);
        Ok(form.clone())
    }
    // pub fn get_all_monitors_of_user(
    //     token: &str,
    //     conn: &PgConnection,
    // ) -> Result<Vec<Monitor>, DbError> {
    //     let uid = crate::users::model::User::get_uid_from_token(token, conn);
    //     use crate::schema::monitors::dsl::*;

    //     let all_monitors = monitors
    //         .select((
    //             id,
    //             id_agent,
    //             id_lambda,
    //             id_organization,
    //             name,
    //             aws_eventbridge_region,
    //             aws_eventbridge_name,
    //             aws_eventbridge_description,
    //             aws_eventbridge_event_bus_name,
    //             aws_eventbridge_schedule_expression,
    //         ))
    //         .load::<Monitor>(conn)
    //         .unwrap();
    //     Ok(all_monitors)
    // }
}
