mod config;
mod handlers;
mod response;
mod route;
mod structs;
mod utils;

use config::Config;
use std::sync::Arc;

use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};
use dotenv::dotenv;
use route::create_router;
use tower_http::cors::CorsLayer;

use mongodb::{options::ClientOptions, Client};

#[derive(Debug)]
pub struct AppState {
    env: Config,
    db: mongodb::Database,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let config = Config::init();

    let database_url = &config.database_url;
    let database_name = &config.database_name;
    // A Client is needed to connect to MongoDB:
    let client_options = ClientOptions::parse(database_url).await.unwrap();
    let client = Client::with_options(client_options).unwrap();

    let db = client.database(database_name);

    // Print the databases in our MongoDB cluster:
    println!("📙 Databases:");
    for name in client.list_database_names(None, None).await.unwrap() {
        println!("- {}", name);
    }

    // Print the collections in our database:
    println!("📌 Collections:");
    for collection_name in db.list_collection_names(None).await.unwrap() {
        println!("- {}", collection_name);
    }

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app = create_router(Arc::new(AppState {
        db: db,
        env: config.clone(),
    }))
    .layer(cors);

    println!("🚀 Server started successfully");
    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
