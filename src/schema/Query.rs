use std::error::Error;

use async_graphql::Object;

use crate::{create_pool, models::User};

pub struct Query;
#[Object]
impl Query {
    async fn users(&self) -> Result<Vec<User>, String> {
        if let Ok(users) = backend::get_users() {
            return Ok(users.into_iter().map(|u| u.into()).collect());
        }
        Err("Error getting users".to_string())
    }
    async fn user(&self, email: String) -> Result<User,String> {
        let mut conn = create_pool().get().unwrap();

        if let Ok(x) = backend::get_user_by_email(&mut conn, email) {
            return Ok(x.into())
        }

        Err(String::from("Error fetching user"))
    }
}
