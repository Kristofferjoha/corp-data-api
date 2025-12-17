use serde::{Serialize, Deserialize};

use crate ::dto::employee_dto::{CreateEmployeeRequest, EmployeeResponse};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Employee {
    pub id: Option<i32>, // optional as it will be set by the database
    pub name: String, // name of the employee
    pub office_id: i32, // foreign key to the office
}

impl Employee {
    // Converts a CreateEmployeeRequest DTO into an Employee entity
    pub fn from_create_request(req: CreateEmployeeRequest) -> Self {
        Employee {
            id: None,
            name: req.name,
            office_id: req.office_id,
        }
    }
    // Converts the Office entity into an OfficeResponse DTO
    pub fn to_response(&self) -> EmployeeResponse {
        EmployeeResponse {
            id: self.id,
            name: self.name.clone(),
            office_id: self.office_id,
        }
    }
}