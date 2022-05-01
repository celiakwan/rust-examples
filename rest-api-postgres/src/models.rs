use super::db;
use super::errors::ServiceError;
use super::schema::users::dsl::users as users_table;
use crate::schema::*;
use chrono::{NaiveDateTime};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Queryable, Debug)]
pub struct User {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub created_at: NaiveDateTime
}

#[derive(Deserialize, Serialize, Insertable, Debug)]
#[table_name = "users"]
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String
}

impl User {
    pub fn find_all() -> Result<Vec<User>, ServiceError> {
        let conn = db::connection()?;
        Ok(users_table.load::<User>(&conn)?)
    }
    
    pub fn find(user_id: Uuid) -> Result<User, ServiceError> {
        let conn = db::connection()?;
        Ok(users_table.find(user_id).get_result::<User>(&conn)?)
    }
    
    pub fn create(new_user: NewUser) -> Result<User, ServiceError> {
        let conn = db::connection()?;
        Ok(diesel::insert_into(users_table).values(&new_user).get_result(&conn)?)
    }
    
    pub fn delete(user_id: Uuid) -> Result<usize, ServiceError> {
        let conn = db::connection()?;
        Ok(diesel::delete(users_table.find(user_id)).execute(&conn)?)
    }
}
