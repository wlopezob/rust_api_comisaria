use serde::{Serialize, Deserialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DistritoResponse {
    pub features: Option<Vec<Feature>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Feature {
    pub attributes: Attributes,
    #[serde(skip)]
    pub geometry: Geometry,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    pub objectid: i64,
    #[serde(rename = "id_dpto")]
    pub id_dpto: String,
    pub departamento: String,
    #[serde(rename = "id_prov")]
    pub id_prov: String,
    pub provincia: String,
    #[serde(rename = "id_dist")]
    pub id_dist: String,
    pub distrito: String,
    pub capital: String,
    pub fuente: String,
    #[serde(rename = "st_area(shape)")]
    pub st_area_shape: f64,
    #[serde(rename = "st_length(shape)")]
    pub st_length_shape: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Geometry {
    pub rings: Vec<Vec<Vec<i64>>>,
}