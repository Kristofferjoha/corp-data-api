use serde::{Serialize, Deserialize};

use crate::dto::office_dto::{CreateOfficeRequest, OfficeResponse};

/// Office entity 
/// Represents an office with an optional ID, name, and maximum occupancy.
/// 
/// database schema:
/// id SERIAL PRIMARY KEY,
/// name TEXT NOT NULL UNIQUE,
/// max_occupancy INT NOT NULL CHECK (max_occupancy > 0)


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Office {
    pub id: Option<i32>, // optional as it will be set by the database
    pub name: String, // name of the office, unique
    pub max_occupancy: i32, // maximum occupancy of the office
}

impl Office {
    // Converts a CreateOfficeRequest DTO into an Office entity
    pub fn from_create_request(req: CreateOfficeRequest) -> Self {
        Office {
            id: None,
            name: req.name.trim().to_string(),
            max_occupancy: req.max_occupancy,
        }
    }
    // Converts the Office entity into an OfficeResponse DTO
    pub fn to_response(&self) -> OfficeResponse {
        OfficeResponse {
            id: self.id,
            name: self.name.clone(),
            max_occupancy: self.max_occupancy,
        }
    }

    // validates the Office entity
    // Used when updating or creating office
    pub fn validate(&self) -> anyhow::Result<()> {
        if self.max_occupancy <= 0 {
            return Err(anyhow::anyhow!("Max occupancy must be greater than 0"));
        }
        if self.name.trim().is_empty() {
            return Err(anyhow::anyhow!("Name can not be empty"));
        }
        Ok(())
    }
}