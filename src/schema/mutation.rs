use crate::{
    create_pool,
    models::{User, UserTest},
};

// use super::Requests::ReqInput;
use async_graphql::{Context, Error, Object, Result};
use async_graphql_postgres::get_pubsub_from_ctx;
// use sqlx::{Pool, Postgres};

pub struct Mutation;

#[Object]
impl Mutation {
    pub async fn users(&self) -> Users {
        Users
    }
}

pub struct Users;

#[Object]
impl Users {
    pub async fn signup(&self, ctx: &Context<'_>, id: i32, email: String) -> Result<String> {
        let mut pub_sub = get_pubsub_from_ctx::<UserTest>(ctx).await?.clone();
        println!("Signing up: {}", &email);

        pub_sub
            .publish(format!("{}", id), UserTest { id, name: email })
            .await;
        Ok("User added successfully".to_string())
    }

    pub async fn login(&self, username: String, password: String) -> Result<User> {
        // let mut conn = create_pool().get().unwrap();
        println!("Logging in");
        let email = username;

        // if let Ok(x) = backend::login(&mut conn, email, Some(password)) {
        //     return Ok(x.into());
        // }
        Err("Could not log in".to_string().into())
    }
    // pub async fn signup(
    //     &self,
    //     username: String,
    //     password: String,
    //     email: String,
    // ) -> Result<String> {
    //     let mut conn = create_pool().get().unwrap();

    //     if backend::add_user(&mut conn, Some(username), Some(password), email).is_ok() {
    //         return Ok("User added successfully".to_string());
    //     }
    //     Ok("Could not sign up!".to_string())
    // }
}
