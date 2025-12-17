use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateOfficeRequest {
    pub name: String,
    pub max_occupancy: i32,
}

#[derive(Debug, Serialize)]
pub struct OfficeResponse {
    pub id: Option<i32>,
    pub name: String,
    pub max_occupancy: i32,
}