use crate::diesel::RunQueryDsl;
use crate::schema::roles;
use diesel::prelude::*;
use diesel::{AsChangeset, Queryable};
use serde_derive::{Deserialize, Serialize};

type DbError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Serialize, Deserialize, Queryable, AsChangeset, Insertable)]
#[table_name = "roles"]
pub struct Role {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[table_name = "roles"]
pub struct InsertableRole {
    pub name: String,
}

impl Role {
    pub const USER: i32 = 1;
    pub const ADMIN: i32 = 2;
    /// Run query using Diesel to insert a new database row and return the result.
    pub fn insert_new_role(
        // prevent collision with `name` column imported inside the function
        form: &InsertableRole,
        conn: &PgConnection,
    ) -> Result<InsertableRole, DbError> {
        // It is common when using Diesel with Actix Web to import schema-related
        // modules inside a function's scope (rather than the normal module's scope)
        // to prevent import collisions and namespace pollution.
        use crate::schema::roles::dsl::*;

        diesel::insert_into(roles).values(form).execute(conn)?;

        Ok(form.clone())
    }
    pub fn get_all_roles(
        // prevent collision with `name` column imported inside the function
        conn: &PgConnection,
    ) -> Result<Vec<Role>, DbError> {
        // It is common when using Diesel with Actix Web to import schema-related
        // modules inside a function's scope (rather than the normal module's scope)
        // to prevent import collisions and namespace pollution.
        use crate::schema::roles::dsl::*;

        // diesel::insert_into(roles).values(form).execute(conn)?;
        let all_roles = roles.load::<Role>(conn).unwrap();
        // roles.load::<Role>(conn).unwrap()

        Ok(all_roles)
    }
}
