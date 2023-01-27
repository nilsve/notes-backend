use actix_cors::Cors;
use actix_web::http::header;
use actix_web::web::Data;
use actix_web::{middleware, web, HttpResponse};
use aws_sdk_dynamodb::Client;

use juniper::{graphql_object, EmptySubscription, FieldResult, GraphQLEnum};
use juniper_actix::graphql_handler;
use lambda_web::actix_web::{self, App, HttpServer};
use lambda_web::{is_running_on_lambda, run_actix_on_lambda, LambdaError};
use shared::db::entities::customer::customer_data::{create_customer, get_customer_entities};

use shared::dto::customer::{Customer, NewCustomer};

#[derive(GraphQLEnum)]
enum Episode {
    NewHope,
    Empire,
    Jedi,
}

// Now, we create our root Query and Mutation types with resolvers by using the
// object macro.
// Objects can have contexts that allow accessing shared state like a database
// pool.

struct Context {
    client: Client,
}

impl Context {
    async fn new() -> Self {
        let shared_config = aws_config::load_from_env().await;
        let client = Client::new(&shared_config);

        Self { client }
    }
}

// To make our context usable by Juniper, we have to implement a marker trait.
impl juniper::Context for Context {}

struct Query;

#[graphql_object(
// Here we specify the context type for the object.
// We need to do this in every type that
// needs access to the context.
context = Context,
)]
impl Query {
    fn apiVersion() -> &'static str {
        "1.0"
    }

    // Arguments to resolvers can either be simple types or input objects.
    // To gain access to the context, we specify a argument
    // that is a reference to the Context type.
    // Juniper automatically injects the correct context here.
    async fn customer(_context: &Context, _new_customer: NewCustomer) -> FieldResult<Customer> {
        unimplemented!()
    }

    async fn all_customers(context: &Context) -> FieldResult<Vec<Customer>> {
        let customers = get_customer_entities(&context.client).await?;

        Ok(customers
            .into_iter()
            .map(Customer::from)
            .collect())
    }
}

// Now, we do the same for our Mutation type.

struct Mutation;

#[graphql_object(
context = Context
)]
impl Mutation {
    async fn createCustomer<S: ScalarValue + Display>(
        context: &Context,
        new_customer: NewCustomer,
    ) -> FieldResult<Customer> {
        let customer_entity = create_customer(&context.client, new_customer).await?;

        Ok(customer_entity.into())
    }
}

// A root schema consists of a query, a mutation, and a subscription.
// Request queries can be executed against a RootNode.
type Schema = juniper::RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

async fn graphql_route(
    req: actix_web::HttpRequest,
    payload: web::Payload,
    schema: Data<Schema>,
) -> Result<HttpResponse, actix_web::Error> {
    let context = Context::new().await;

    graphql_handler(&schema, &context, req, payload).await
}

fn schema() -> Schema {
    Schema::new(Query, Mutation {}, EmptySubscription::new())
}

#[actix_web::main]
async fn main() -> Result<(), LambdaError> {
    let factory = move || {
        App::new()
            .app_data(Data::new(schema()))
            .wrap(
                Cors::default()
                    .allow_any_origin()
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
            .bind("127.0.0.1:3000")?
            .run()
            .await?;
    }
    Ok(())
}
