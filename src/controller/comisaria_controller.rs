use axum::{
    routing::{get},
    Router,
};

use crate::routes::init::AppState;

pub fn comisaria_controller() -> Router<AppState> {
    Router::new().route("/getall", get(get_all))
}

async fn get_all() -> String {
    "comisaria".to_owned()
}