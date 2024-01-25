use crate::{create_pool, models::User};
use async_graphql::{InputObject, Object, Result};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

#[derive(Default)]
pub struct Login;

#[derive(InputObject)]
pub struct ILogin {
    email: String,
    password: String,
}

#[Object]
impl Login {
    pub async fn login(&self, input: ILogin) -> Result<User> {
        use crate::schema::users::dsl::{email as mail, password as pass, users};

        let mut conn = create_pool().get().unwrap();
        println!("Logging in");
        let user = users
            .filter(mail.eq(input.email))
            .filter(pass.eq(input.password))
            .get_result::<User>(&mut conn)
            .map_err(|e| {
                eprintln!("User not found: {}", e);
                e
            })?;

        Ok(user)
    }
}
