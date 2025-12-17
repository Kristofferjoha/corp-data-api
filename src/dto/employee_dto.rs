use serde::{Deserialize, Serialize};

// Data Transfer Object for creating a new employee
#[derive(Debug, Deserialize)]
pub struct CreateEmployeeRequest {
    pub name: String,
    pub office_id: i32,
}

// Data Transfer Object for employee GET responses
#[derive(Debug, Serialize)]
pub struct EmployeeResponse {
    pub id: Option<i32>,
    pub name: String,
    pub office_id: i32,
}