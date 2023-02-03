use axum::{
    routing::{get},
    Router,
};

pub fn comisaria_controller() -> Router {
    Router::new().route("/getall", get(get_all))
}

async fn get_all() -> String {
    "comisaria".to_owned()
}