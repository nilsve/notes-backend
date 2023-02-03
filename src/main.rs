use actix_cors::Cors;
use actix_web::http::header;
use actix_web::web::Data;
use actix_web::{middleware, web, HttpResponse};

use crate::resolver::mutation::Mutation;
use crate::resolver::query::Query;
use crate::resolver::{Context, Schema};
use juniper::EmptySubscription;
use juniper_actix::graphql_handler;
use lambda_web::actix_web::{self, App, HttpServer};
use lambda_web::{is_running_on_lambda, run_actix_on_lambda, LambdaError};

pub mod dto;
pub mod repository;

mod resolver;

async fn graphql_route(
    req: actix_web::HttpRequest,
    payload: web::Payload,
    schema: Data<Schema>,
    context: Data<Context>,
) -> Result<HttpResponse, actix_web::Error> {
    graphql_handler(&schema, &context, req, payload).await
}

fn schema() -> Schema {
    Schema::new(Query, Mutation {}, EmptySubscription::new())
}

#[actix_web::main]
async fn main() -> Result<(), LambdaError> {
    let context = Context::new().await;

    let factory = move || {
        App::new()
            .app_data(Data::new(context.clone()))
            .app_data(Data::new(schema()))
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    // .send_wildcard()
                    .allowed_methods(vec!["POST", "GET"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(
                web::resource("/graphql")
                    .route(web::post().to(graphql_route))
                    .route(web::get().to(graphql_route)),
            )
    };

    if is_running_on_lambda() {
        // Run on AWS Lambda
        run_actix_on_lambda(factory).await?;
    } else {
        // Local server
        HttpServer::new(factory)
            .bind("127.0.0.1:3123")?
            .run()
            .await?;
    }
    Ok(())
}
