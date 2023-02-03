use axum::{
    routing::{get},
    Router,
};

pub fn home_controller() -> Router {
    Router::new().route("/", get(home))
}

async fn home() -> String {
    "hola mundo".to_owned()
}
