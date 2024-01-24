use crate::{
    create_pool,
    models::{Message, NewMessage, NewUser, User},
};

use crate::pubsub::get_pubsub_from_ctx;
use async_graphql::{Context, InputObject, Object, Result};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

pub struct Mutation;

#[Object]
impl Mutation {
    pub async fn users(&self) -> Users {
        Users
    }
    pub async fn messages(&self) -> Messages {
        Messages
    }
}

pub struct Users;

#[derive(InputObject)]
pub struct ISignup {
    email: String,
    password: String,
    username: Option<String>,
}
#[derive(InputObject)]
pub struct ILogin {
    email: String,
    password: String,
}

#[Object]
impl Users {
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

pub struct Messages;

#[derive(InputObject)]
pub struct IAddMessage {
    sender: i32,
    receiver: i32,
    text: String,
}

#[Object]
impl Messages {
    pub async fn send_message(&self, ctx: &Context<'_>, input: IAddMessage) -> Result<Message> {
        use crate::schema::messages::dsl::messages;
        let mut conn = create_pool().get().unwrap();
        let mut pub_sub = get_pubsub_from_ctx::<Message>(ctx).await?.clone();
        println!("Sending message to: {}", &input.receiver);

        let message = diesel::insert_into(messages)
            .values(NewMessage {
                sender: input.sender,
                receiver: input.receiver,
                text: input.text,
            })
            .get_result::<Message>(&mut conn)
            .map_err(|e| {
                eprintln!("ERROR: Failed to send message: {}", e);
                e
            })?;

        pub_sub
            .publish("new_message".to_string(), message.clone())
            .await;

        Ok(message)
    }
}
