pub mod errors;
pub mod graphql_schema;
pub mod models;
pub mod pubsub;
mod schema;

use async_graphql::{extensions::Analyzer, http::GraphiQLSource, Schema};
use async_graphql_actix_web::GraphQLSubscription;
use diesel::PgConnection;
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;
use diesel_migrations::FileBasedMigrations;
use errors::ApplicationError;
use graphql_schema::Mutation;
use graphql_schema::Query;
use graphql_schema::Subscription;
use models::{Message, User};
use pubsub::PubSubHandler;
use tokio::sync::Mutex;

use std::env;

use actix_web::{web, HttpRequest, HttpResponse, Result};

use diesel::{migration::MigrationSource, Connection};
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

pub fn create_pool() -> Pool<AsyncPgConnection> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);

    Pool::builder(config)
        .build()
        .expect("Could not build pool!")
}

pub fn build_schema() -> AppSchema {
    Schema::build(
        Query::default(),
        Mutation::default(),
        Subscription::default(),
    )
    .data(Mutex::new(PubSubHandler::<User>::default()))
    .data(Mutex::new(PubSubHandler::<Message>::default()))
    .extension(Analyzer)
    .enable_federation()
    .finish()
}

pub fn run_migrations() -> Result<(), ApplicationError> {
    let database_url = dotenvy::var("DATABASE_URL").expect("database Url not set");

    let mut db_connection = PgConnection::establish(&database_url)?;
    let mut migrations = FileBasedMigrations::from_path("./backend/migrations")?
        .migrations()
        .unwrap();

    migrations.sort_by_key(|m| m.name().to_string());

    for migration in migrations {
        migration.run(&mut db_connection).unwrap();
    }

    Ok(())
}
