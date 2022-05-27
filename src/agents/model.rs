use diesel::prelude::*;

use crate::diesel::RunQueryDsl;
use crate::schema::agents;
use diesel::{AsChangeset, Queryable};
use rand::{distributions::Alphanumeric, Rng};
use serde_derive::{Deserialize, Serialize};

type DbError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Serialize, Deserialize, Queryable, AsChangeset, Insertable)]
#[table_name = "agents"]
pub struct Agent {
    pub id: i32,
    pub name: String,
    pub token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[table_name = "agents"]
pub struct InsertableAgent {
    pub name: String,
    pub token: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AgentName {
    pub name: String,
}

impl InsertableAgent {
    fn from_agent(agent: Agent) -> InsertableAgent {
        InsertableAgent {
            name: agent.name,
            token: agent.token,
        }
    }
}

impl Agent {
    pub fn insert_new_agent(nm: &String, conn: &PgConnection) -> Result<InsertableAgent, DbError> {
        use crate::schema::agents::dsl::*;

        let s: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(70)
            .map(char::from)
            .collect();

        let new_agent = InsertableAgent {
            name: nm.to_owned(),
            token: s,
        };

        diesel::insert_into(agents)
            .values(&new_agent)
            .execute(conn)?;

        Ok(new_agent)
    }
}
