use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Data Transfer Object for creating a new office
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateOfficeRequest {
    pub name: String,
    pub max_occupancy: i32,
}

/// Data Transfer Object for office GET responses
#[derive(Debug, Serialize, ToSchema)]
pub struct OfficeResponse {
    pub id: Option<i32>,
    pub name: String,
    pub max_occupancy: i32,
}