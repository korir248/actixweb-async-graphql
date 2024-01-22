use async_graphql::{Object, Result};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

use crate::{create_pool, models::User};

pub struct Query;

#[Object]
impl Query {
    async fn users(&self) -> Result<Vec<User>> {
        use crate::schema::users::dsl::users;
        let mut conn = create_pool().get().unwrap();

        let user_list = users.load::<User>(&mut conn)?;

        Ok(user_list)
    }
    async fn user(&self, email: String) -> Result<User> {
        use crate::schema::users::dsl::{email as mail, users};

        let mut conn = create_pool().get().unwrap();

        let user = users.filter(mail.eq(email)).get_result::<User>(&mut conn)?;

        Ok(user)
    }
}
