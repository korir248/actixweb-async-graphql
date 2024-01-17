mod models;
mod schema;
use std::{env, str::FromStr};

use actix_cors::Cors;
use actix_web::{
    guard,
    http::header::{self},
    web,
    web::Data,
    App, HttpResponse, HttpServer, Result,
};
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription, Schema,
};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use dotenvy::dotenv;
use reqwest::header::{HeaderMap, HeaderValue};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let schema = Schema::build(
        schema::Query::Query,
        schema::Mutation::Mutation,
        EmptySubscription,
    )
    .enable_federation()
    .finish();
    println!("GraphiQL IDE: http://localhost:4000/users/graphql");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:4500")
            .allowed_origin("http://localhost:4000")
            .allow_any_method()
            .supports_credentials()
            .allow_any_header();
        App::new()
            .wrap(cors)
            .app_data(Data::new(schema.clone()))
            .service(
                web::resource("/users/graphql")
                    .guard(guard::Post())
                    .to(index),
            )
            .service(
                web::resource("/users/graphql")
                    .guard(guard::Get())
                    .to(index_graphiql),
            )
    })
    .bind("0.0.0.0:4000")?
    .run()
    .await
}

async fn index(
    schema: web::Data<Schema<schema::Query::Query, schema::Mutation::Mutation, EmptySubscription>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let mut headers = HeaderMap::new();
    headers.insert(
        header::ORIGIN,
        HeaderValue::from_str("http://localhost:4500").unwrap(),
    );
    schema
        .execute(req.into_inner())
        .await
        .http_headers(headers)
        .into()
}
// to use GraphiQL instead
//.body(GraphiQLSource::build().endpoint("/").finish()))

async fn index_graphiql() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(GraphQLPlaygroundConfig::new(
            "http://localhost:4000/users/graphql",
        ))))
}

pub fn create_pool() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);

    Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}
