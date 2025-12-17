use serde::{Serialize, Deserialize};

use crate::dto::office_dto::{CreateOfficeRequest, OfficeResponse};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Office {
    pub id: Option<i32>,
    pub name: String,
    pub max_occupancy: i32,
}

impl Office {
    pub fn from_create_request(req: CreateOfficeRequest) -> Self {
        Office {
            id: None,
            name: req.name,
            max_occupancy: req.max_occupancy,
        }
    }

    pub fn to_response(&self) -> OfficeResponse {
        OfficeResponse {
            id: self.id,
            name: self.name.clone(),
            max_occupancy: self.max_occupancy,
        }
    }
}