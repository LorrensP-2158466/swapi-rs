mod starwars;

use async_graphql::dataloader::*;
use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema};
use async_graphql_axum::GraphQL;
use axum::{
    response::{self, IntoResponse},
    routing::get,
    Router,
};
use sqlx::PgPool;
use starwars::{credits_loader::CreditsDataLoader, MutationRoot, QueryRoot, StarWarsAPI};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/").finish())
}

#[tokio::main]
async fn main() {
    let swapi = StarWarsAPI::default();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await.unwrap();

    // Run migrations automatically when the application starts
    sqlx::migrate!("./migrations") // Path to the migrations folder
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(swapi)
        .data(pool.clone()) // the database connection
        .data(DataLoader::new(
            CreditsDataLoader { pool: pool.clone() },
            tokio::task::spawn,
        ))
        //.data(DatabasePool) // kunt een database toevoegen
        //.data(FacebookAPI) // kunt een api toevoegen
        //.data(s3Bucket) // s3 buckets
        // ...etc
        .finish();

    let app = Router::new()
        .route("/", get(graphiql).post_service(GraphQL::new(schema)))
        .layer(TraceLayer::new_for_http());

    println!("GraphiQL IDE: http://localhost:8000");

    axum::serve(TcpListener::bind("0.0.0.0:8000").await.unwrap(), app)
        .await
        .unwrap();
}
