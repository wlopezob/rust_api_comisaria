use axum::Router;
use dotenvy_macro::dotenv;

use crate::controller::{home_controller::home_controller, ubigeo_controller::ubigeo_controller, comisaria_controller::comisaria_controller};

pub async fn run() {
    let port = dotenv!("SERVER_PORT");
    let app = init_router();

    axum::Server::bind(&format!("0.0.0.0:{port}").parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn init_router() -> Router {
    Router::new().nest("/api/v1", chidren_router())
}

fn chidren_router() -> Router {
    Router::new()
    .nest("/home", home_controller())
    .nest("/ubigeo", ubigeo_controller())
    .nest("/comisaria", comisaria_controller())
}
