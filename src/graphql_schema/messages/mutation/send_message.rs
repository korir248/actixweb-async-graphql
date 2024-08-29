use async_graphql::{Context, InputObject, Object, Result};
use diesel_async::RunQueryDsl;

use crate::{
    create_pool,
    models::{Message, NewMessage},
    pubsub::get_pubsub_from_ctx,
};

#[derive(Default)]
pub struct SendMessage;

#[derive(InputObject)]
pub struct IAddMessage {
    sender: i32,
    receiver: i32,
    text: String,
}

#[Object]
impl SendMessage {
    pub async fn send_message(&self, ctx: &Context<'_>, input: IAddMessage) -> Result<Message> {
        use crate::schema::messages::dsl::messages;
        let mut conn = create_pool().get().await?;
        let mut pub_sub = get_pubsub_from_ctx::<Message>(ctx).await?.clone();
        println!("Sending message to: {}", &input.receiver);

        let message = diesel::insert_into(messages)
            .values(NewMessage {
                sender: input.sender,
                receiver: input.receiver,
                text: input.text,
            })
            .get_result::<Message>(&mut conn)
            .await
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
