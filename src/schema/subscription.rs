use async_graphql::{Result, Subscription};
use futures_util::Stream;
use std::time::Duration;

pub struct Subscription;

#[Subscription]
impl Subscription {
    pub async fn interval(&self, n: i32) -> impl Stream<Item = String> {
        async_stream::stream! {
         let mut interval = tokio::time::interval(Duration::from_secs(2));
         loop {
             let n = interval.tick().await;
             yield format!("{}", n.elapsed().as_secs());
         }
        }
    }
}
