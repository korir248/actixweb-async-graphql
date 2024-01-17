use async_graphql::{Result, Subscription};
use futures_util::Stream;

pub struct Subscription;

#[Subscription]
impl Subscription {
    pub async fn interval(&self, n: i32) -> impl Stream<Item = i32> {
        futures_util::stream::iter(0..n)
    }
}
