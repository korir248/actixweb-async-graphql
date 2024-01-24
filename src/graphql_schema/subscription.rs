use crate::pubsub::{get_pubsub_from_ctx, StreamResult};
use async_graphql::{Context, InputObject, MergedSubscription, Subscription};
use futures_util::Stream;

use crate::models::{Message, User};

#[derive(Default, MergedSubscription)]
pub struct Subscription(pub GetNewUser, pub GetNewMessage);

#[derive(Default)]
pub struct GetNewUser;

#[Subscription]
impl GetNewUser {
    pub async fn new_user<'a>(
        &'a self,
        ctx: &'a Context<'a>,
        channel: String,
    ) -> impl Stream<Item = StreamResult<User>> + 'a {
        let mut pub_sub = get_pubsub_from_ctx::<User>(ctx).await.unwrap();

        pub_sub.subscribe(channel).await
    }
}

#[derive(Default)]
pub struct GetNewMessage;

#[derive(InputObject)]
pub struct IGetNewMessage {
    channel: String,
}

#[Subscription]
impl GetNewMessage {
    pub async fn new_message<'a>(
        &'a self,
        ctx: &'a Context<'a>,
        input: IGetNewMessage,
    ) -> impl Stream<Item = StreamResult<Message>> + 'a {
        let mut pub_sub = get_pubsub_from_ctx::<Message>(ctx).await.unwrap();

        pub_sub.subscribe(input.channel).await
    }
}
