use async_graphql::{Context, Result, Subscription};
use async_graphql_postgres::{get_pubsub_from_ctx, PubSubHandler};
use futures_util::Stream;
use std::{sync::Mutex, time::Duration};

use crate::models::UserTest;

pub struct Subscription;

#[Subscription]
impl Subscription {
    pub async fn new_user<'a>(
        &'a self,
        ctx: &'a Context<'a>,
        id: i32,
    ) -> impl Stream<Item = UserTest> + 'a {
        let mut pub_sub = get_pubsub_from_ctx::<UserTest>(ctx).await.unwrap();

        async_stream::stream! {
            let mut rx = pub_sub.subscribe(format!("{}", id)).await;
            loop {
                match rx.try_recv() {
                    Ok(val) =>{println!("Obtained value: {:#?}",val.clone()); yield val},
                    Err(e) =>{ eprintln!("{}",e);
                    continue},
                }
            }
        }
    }
}
