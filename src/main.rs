use actix_cors::Cors;
use actix_web::{guard, web, App, HttpServer};
use async_graphql_actix_web::GraphQL;
use async_graphql_postgres::{build_schema, index_graphiql, index_ws};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let schema = build_schema();

    println!("GraphiQL IDE: http://localhost:4000/");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:4500")
            .allowed_origin("http://localhost:4000")
            .allow_any_method()
            .supports_credentials()
            .allow_any_header();
        App::new()
            .wrap(cors)
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
