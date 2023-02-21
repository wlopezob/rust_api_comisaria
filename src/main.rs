use dotenvy::dotenv;
mod routes;
mod controller;
mod models;
mod utils;
mod api_caller;
mod services;
mod db;
mod repository;
mod producer;
use crate::routes::init::run;
use crate::utils::error::Error;
pub type Result<T> = core::result::Result<T, Error>;

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    run().await;
}
