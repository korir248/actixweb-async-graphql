#![allow(unused)]
mod models;
mod schema;
use schema::mutation::Mutation;
use schema::query::Query;
use schema::subscription::Subscription;

use std::env;

use actix_cors::Cors;
use actix_web::{guard, web, App, HttpRequest, HttpResponse, HttpServer, Result};
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig, GraphiQLSource},
    EmptySubscription, Schema,
};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse, GraphQLSubscription, GraphQL};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use dotenvy::dotenv;
use reqwest::header::{HeaderMap, HeaderValue};

type AppSchema = Schema<Query, Mutation, Subscription>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let schema: AppSchema = Schema::build(Query, Mutation, Subscription)
        .enable_federation()
        .finish();
    println!("GraphiQL IDE: http://localhost:4000/");

    HttpServer::new(move || {
        let _cors = Cors::default()
            .allowed_origin("http://localhost:4500")
            .allowed_origin("http://localhost:4000")
            .allow_any_method()
            .supports_credentials()
            .allow_any_header();
        App::new()
            // .wrap(cors)
            .app_data(web::Data::new(schema.clone()))
            .service(
                web::resource("/")
                    .guard(guard::Post())
                    .to(GraphQL::new(schema.clone())),
            )
            .service(
                web::resource("/")
                    .guard(guard::Get())
                    .guard(guard::Header("upgrade", "websocket"))
                    .to(index_ws),
            )
            .service(web::resource("/").guard(guard::Get()).to(index_graphiql))
    })
    .bind("0.0.0.0:4000")?
    .run()
    .await
}

async fn index_graphiql() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf8")
        .body(
            GraphiQLSource::build()
                .endpoint("/")
                .subscription_endpoint("/")
                .finish(),
        ))
}
async fn index_ws(
    schema: web::Data<AppSchema>,
    req: HttpRequest,
    payload: web::Payload,
) -> Result<HttpResponse> {
    GraphQLSubscription::new(Schema::clone(&*schema)).start(&req, payload)
}
// async fn index(schema: web::Data<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
//     let mut headers = HeaderMap::new();
//     headers.insert(
//         header::ORIGIN,
//         HeaderValue::from_str("http://localhost:4500").unwrap(),
//     );
//     schema
//         .execute(req.into_inner())
//         .await
//         .http_headers(headers)
//         .into()
// }
// to use GraphiQL instead
//.body(GraphiQLSource::build().endpoint("/").finish()))

// async fn index_graphiql() -> Result<HttpResponse> {
//     Ok(HttpResponse::Ok()
//         .content_type("text/html; charset=utf-8")
//         .body(playground_source(GraphQLPlaygroundConfig::new(
//             "http://localhost:4000/",
//         ))))
// }

pub fn create_pool() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);

    Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}
