use dotenvy::dotenv;
mod routes;
mod controller;
mod models;
mod utils;
use crate::routes::init::run;

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    run().await;
}
