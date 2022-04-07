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

// #[derive(Insertable)]
// #[table_name = "posts"]
// pub struct InsertablePost {
//     pub url: String,
//     pub title: String,
//     pub description: Option<String>,
//     pub image_url: String,
//     pub date: String,
//     pub length: Option<i16>,
// }

// impl InsertablePost {
//     fn from_post(post: Post) -> InsertablePost {
//         InsertablePost {
//             url: post.url,
//             title: post.title,
//             description: post.description,
//             image_url: post.image_url,
//             date: post.date,
//             length: post.length,
//         }
//     }
// }

impl Metric {
    // pub fn create(post: Post, connection: &PgConnection) -> QueryResult<Post> {
    //     diesel::insert_into(posts::table)
    //         .values(&InsertablePost::from_post(post))
    //         .execute(connection)?;
    //     posts::table.order(posts::id.desc()).first(connection)
    // }

    pub fn get_all(connection: &PgConnection) -> QueryResult<Vec<Metric>> {
        metrics::table.order(metrics::id).load::<Metric>(connection)
    }
}
