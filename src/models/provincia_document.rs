use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct ProvinciaDocument {
    id_dpto: String,
    id_prov: String,
    provincia: String,
    capital: String,
}

impl ProvinciaDocument {
    pub fn new(id_dpto: String, id_prov: String, provincia: String, capital: String) -> Self {
        ProvinciaDocument {
            id_dpto,
            id_prov,
            provincia,
            capital,
        }
    }
    pub fn get_id_prov(&self) -> &str{
        &self.id_prov
    }
}
