use serde::{Deserialize, Serialize};

/// Data Transfer Object for creating a new office
#[derive(Debug, Deserialize)]
pub struct CreateOfficeRequest {
    pub name: String,
    pub max_occupancy: i32,
}

/// Data Transfer Object for office GET responses
#[derive(Debug, Serialize)]
pub struct OfficeResponse {
    pub id: Option<i32>,
    pub name: String,
    pub max_occupancy: i32,
}