use async_graphql::{Error, Object, Result};

use crate::{create_pool, models::User};

pub struct Query;

#[Object]
impl Query {
    async fn users(&self) -> Result<Vec<User>> {
        // let Ok(users) = backend::get_users() else {
        //     return Err(Error::from("Cant get users"));
        // };
        // Ok(users.into_iter().map(|u| u.into()).collect())
        Err(Error::from("unavailabe"))
    }
    async fn user(&self, email: String) -> Result<User> {
        // let mut conn = create_pool().get().unwrap();

        // let Ok(x) = backend::get_user_by_email(&mut conn, email) else {
        //     return Err(Error::from("Error fetching user"));
        // };

        // Ok(x.into())
        Err(Error::from("unavailabe"))
    }
}
