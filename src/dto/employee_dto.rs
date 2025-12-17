use serde::{Deserialize, Serialize};
use chrono::NaiveDate;

// Data Transfer Object for creating a new employee
#[derive(Debug, Deserialize)]
pub struct CreateEmployeeRequest {
    pub first_name: String,
    pub last_name: String,
    pub birth_date: NaiveDate,
    pub office_id: i32,
}

// Data Transfer Object for employee GET responses
#[derive(Debug, Serialize)]
pub struct EmployeeResponse {
    pub id: Option<i32>,
    pub first_name: String,
    pub last_name: String,
    pub birth_date: NaiveDate,
    pub office_id: i32,
}