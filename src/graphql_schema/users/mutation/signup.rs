use crate::{
    create_pool,
    models::{NewUser, User},
    pubsub::get_pubsub_from_ctx,
};
use async_graphql::{Context, InputObject, Object, Result};
use diesel::RunQueryDsl;

#[derive(Default)]
pub struct Signup;

#[derive(InputObject)]
pub struct ISignup {
    email: String,
    password: String,
    username: Option<String>,
}

#[Object]
impl Signup {
    pub async fn signup(&self, ctx: &Context<'_>, input: ISignup) -> Result<User> {
        use crate::schema::users::dsl::users;
        let mut conn = create_pool().get().unwrap();

        let mut pub_sub = get_pubsub_from_ctx::<User>(ctx).await?.clone();
        println!("Signing up: {}", &input.email);

        let user = diesel::insert_into(users)
            .values(NewUser {
                email: input.email,
                password: input.password,
                username: input.username,
            })
            .get_result::<User>(&mut conn)
            .map_err(|e| {
                eprintln!("Failed to register user: {}", e);
                e
            })?;

        pub_sub.publish("new_user".to_string(), user.clone()).await;

        Ok(user)
    }
}
