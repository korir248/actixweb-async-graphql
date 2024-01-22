use crate::pubsub::{get_pubsub_from_ctx, StreamResult};
use async_graphql::{Context, MergedSubscription, Subscription};
use futures_util::Stream;

use crate::models::User;

#[derive(Default, MergedSubscription)]
pub struct Subscription(pub GetNewUser);

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
