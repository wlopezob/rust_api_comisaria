use crate::models::api_exception::ApiException;
use crate::models::departamento_response::DepartamentoResponse;
use crate::{services::ubigeo_service};
use axum::{routing::get, Json, Router};
use dotenvy_macro::dotenv;

pub fn ubigeo_controller() -> Router {
    Router::new().route("/getalldpto", get(get_all_dpto))
}

async fn get_all_dpto() -> Result<Json<Vec<DepartamentoResponse>>, ApiException> {
    let url_dpto = dotenv!("URL_DPTO");
    let host = dotenv!("HOST");
    let origin = dotenv!("ORIGIN");
    let departamentos = ubigeo_service::UbigeoService::new()
        .get_all_dpto(url_dpto, host, origin)
        .await?;
    Ok(Json(departamentos))
}
