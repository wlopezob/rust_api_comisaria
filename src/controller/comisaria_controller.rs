use axum::{extract::State, routing::get, Json, Router};
use dotenvy_macro::dotenv;

use crate::{
    models::{api_exception::ApiException, comisaria_document::ComisariaDocument},
    routes::init::AppState,
    services::comisaria_service::ComisariaService,
};

pub fn comisaria_controller() -> Router<AppState> {
    Router::new().route("/getall", get(get_all))
}

async fn get_all(
    State(app_state): State<AppState>,
) -> Result<Json<Vec<ComisariaDocument>>, ApiException> {
    let url_comisaria_count = dotenv!("URL_COMISARIA_COUNT");
    let url_comisaria = dotenv!("URL_COMISARIA");
    let host = dotenv!("HOST");
    let origin = dotenv!("ORIGIN");

    let comisarias = ComisariaService::new(app_state.comisaria_repository.clone())
        .get_all_comisaria(
            url_comisaria_count.to_owned(),
            url_comisaria.to_owned(),
            host,
            origin,
        )
        .await?;
    Ok(Json(comisarias))
}
