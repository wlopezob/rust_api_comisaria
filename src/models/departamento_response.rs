use serde::{Deserialize, Serialize};

use crate::Result;

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct DepartamentoResponse {
    id_dpto: String,
    departamento: String,
    capital: String,
}

impl DepartamentoResponse {
    pub fn new() -> Self {
        DepartamentoResponse::default()
    }
    pub fn id_dpto(mut self, id_dpto: impl Into<String>) -> Self {
        self.id_dpto = id_dpto.into();
        self
    }
    pub fn departamento(mut self, departamento: impl Into<String>) -> Self {
        self.departamento = departamento.into();
        self
    }
    pub fn capital(mut self, capital: impl Into<String>) -> Self {
        self.capital = capital.into();
        self
    }

    pub fn builder(self) -> Result<DepartamentoResponse> {
        Ok(DepartamentoResponse {
            id_dpto: self.id_dpto,
            departamento: self.departamento,
            capital: self.capital,
        })
    }
}
