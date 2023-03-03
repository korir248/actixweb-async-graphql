use crate::{create_pool, models::User};

// use super::Requests::ReqInput;
use async_graphql::*;
// use sqlx::{Pool, Postgres};
pub struct Mutation;

#[Object]
impl Mutation {
    pub async fn login(&self, username: String, password: String) -> Result<User> {
        let mut conn = create_pool().get().unwrap();
        println!("Logging in");
        let email = username;

        if let Ok(x) = backend::login(&mut conn, email, Some(password)) {
            return Ok(x.into());
        }
        Err("Could not log in".to_string().into())
    }
    pub async fn signup(
        &self,
        username: String,
        password: String,
        email: String,
    ) -> Result<String> {
        let mut conn = create_pool().get().unwrap();

        if let Ok(_) = backend::add_user(&mut conn, Some(username), Some(password), email) {
            return Ok("User added successfully".to_string());
        }
        Ok("Could not sign up!".to_string())
    }
}
