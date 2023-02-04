use dotenvy::dotenv;
mod routes;
mod controller;
mod models;
mod utils;
mod api_caller;
mod services;
use crate::routes::init::run;

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    run().await;
}
