use axum::{
    routing::{get},
    Router,
};

use crate::routes::init::AppState;

pub fn home_controller() -> Router<AppState> {
    Router::new().route("/", get(home))
}

async fn home() -> String {
    "hola mundo".to_owned()
}
