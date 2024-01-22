use async_graphql::{Context, Result as GraphQLResult};
use async_std::sync::Arc;
use std::collections::{hash_map::Entry, HashMap};
use tokio::sync::{
    broadcast::{channel, Sender},
    Mutex, MutexGuard,
};
use tokio_stream::wrappers::{errors::BroadcastStreamRecvError, BroadcastStream};

pub type StreamResult<T> = Result<T, BroadcastStreamRecvError>;

#[derive(Debug, Clone)]
pub struct PubSubHandler<T>(Arc<Mutex<HashMap<String, Sender<T>>>>);

impl<T: Clone + Send + 'static> PubSubHandler<T> {
    pub fn new() -> Self {
        let map: HashMap<String, Sender<T>> = HashMap::new();

        Self(Arc::new(Mutex::new(map)))
    }
    pub async fn is_empty(&self) -> bool {
        self.0.lock().await.is_empty()
    }

    pub async fn subscribe(&mut self, channel_str: String) -> BroadcastStream<T> {
        println!("Subscribing to: {}", &channel_str);

        let rx = match self.0.lock().await.entry(channel_str) {
            Entry::Occupied(entry) => entry.get().subscribe(),
            Entry::Vacant(e) => {
                let (tx, _rx) = channel::<T>(1024);
                e.insert(tx.clone());
                tx.subscribe()
            }
        };

        BroadcastStream::new(rx)
    }

    pub async fn publish(&mut self, channel_str: String, data: T) {
        println!("Publishing data to: {}", &channel_str);

        let tx = match self.0.lock().await.entry(channel_str) {
            Entry::Occupied(entry) => entry.get().clone(),
            Entry::Vacant(e) => {
                let (tx, _rx) = channel::<T>(1024);
                e.insert(tx.clone());
                tx
            }
        };

        match tx.send(data) {
            Ok(_) => println!("Data sent successfully"),
            Err(e) => eprintln!("Data could not be sent: {e}"),
        }
    }
}

impl<T: Clone + Send + 'static> Default for PubSubHandler<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub async fn get_pubsub_from_ctx<'a, T: Send + Clone + 'static>(
    ctx: &'a Context<'a>,
) -> GraphQLResult<MutexGuard<'a, PubSubHandler<T>>> {
    let pubsub_mutex = ctx.data::<Mutex<PubSubHandler<T>>>().map_err(|e| {
        eprintln!("Pubsub could not be obtained");
        e
    })?;

    let pubsub = pubsub_mutex.lock().await;

    Ok(pubsub)
}
