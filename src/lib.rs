pub mod graphql_schema;
pub mod models;
pub mod pubsub;
mod schema;

use async_graphql::{extensions::Analyzer, http::GraphiQLSource, Schema};
use async_graphql_actix_web::GraphQLSubscription;
use graphql_schema::mutation::Mutation;
use graphql_schema::query::Query;
use graphql_schema::subscription::Subscription;
use models::{Message, User};
use pubsub::PubSubHandler;
use tokio::sync::Mutex;

use std::env;

use actix_web::{web, HttpRequest, HttpResponse, Result};

use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use dotenvy::dotenv;

pub type AppSchema = Schema<Query, Mutation, Subscription>;

pub async fn index_graphiql() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf8")
        .body(
            GraphiQLSource::build()
                .endpoint("/")
                .subscription_endpoint("/")
                .finish(),
        ))
}
pub async fn index_ws(
    schema: web::Data<AppSchema>,
    req: HttpRequest,
    payload: web::Payload,
) -> Result<HttpResponse> {
    GraphQLSubscription::new(Schema::clone(&*schema)).start(&req, payload)
}

pub fn create_pool() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);

    Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

pub fn build_schema() -> AppSchema {
    Schema::build(Query, Mutation, Subscription::default())
        .data(Mutex::new(PubSubHandler::<User>::default()))
        .data(Mutex::new(PubSubHandler::<Message>::default()))
        .extension(Analyzer)
        .enable_federation()
        .finish()
}
