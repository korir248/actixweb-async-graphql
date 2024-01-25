use async_graphql::{Context, InputObject, Subscription};
use futures_util::Stream;

use crate::{
    models::Message,
    pubsub::{get_pubsub_from_ctx, StreamResult},
};

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
