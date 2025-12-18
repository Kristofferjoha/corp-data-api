use serde::{Serialize, Deserialize};

use crate ::dto::employee_dto::{CreateEmployeeRequest, EmployeeResponse};
use crate::utils::Validate;
use chrono::Datelike;

/// Employee entity
/// Represents an employee with an optional ID, first name, last name, birth date, and connected office ID.
/// 
/// database schema:
/// id SERIAL PRIMARY KEY,
/// first_name VARCHAR(100) NOT NULL,
/// last_name VARCHAR(100) NOT NULL,
/// birth_date DATE NOT NULL CHECK (birth_date < CURRENT_DATE),
/// office_id INT NOT NULL REFERENCES offices(id)
/// 
/// Includes validation for last name and vampire/baby status
/// Includes validation tests


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Employee {
    pub id: Option<i32>, // optional as it will be set by the database
    pub first_name: String, // any name but last name 
    pub last_name: String, // last name of the employee
    pub birth_date: chrono::NaiveDate, // birth date of the employee
    pub office_id: i32, // foreign key to office
}

impl Employee {
    // Converts a CreateEmployeeRequest DTO into an Employee entity
    pub fn from_create_request(req: CreateEmployeeRequest) -> Self {
        Employee {
            id: None,
            first_name: req.first_name.trim().to_string(),
            last_name: req.last_name.trim().to_string(),
            birth_date: req.birth_date,
            office_id: req.office_id,
        }
    }
    // Converts the Employee entity into an EmployeeResponse DTO
    pub fn to_response(&self) -> EmployeeResponse {
        EmployeeResponse {
            id: self.id,
            first_name: self.first_name.clone(),
            last_name: self.last_name.clone(),
            birth_date: self.birth_date,
            office_id: self.office_id,
        }
    }
}

// builds on validation trait to validate employee data
impl Validate for Employee {
    fn validate(&self) -> Result<(), String> {
        if self.first_name.trim().is_empty() {
            return Err("First name cannot be empty".into());
        }
        if self.last_name.trim().is_empty() {
            return Err("Last name cannot be empty".into());
        }
        if self.last_name.chars().any(|c| c.is_whitespace()) {
            return Err("Last name cannot contain whitespace".into());
        }
        if self.birth_date.year() < 1919 {
            return Err("Born pre versaille treaty".into());
        }
        if self.birth_date.year() > 2007 {
            return Err("Born post rust".into());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn create_valid_employee() -> Employee {
        Employee {
            id: None,
            first_name: "Kristoffer".to_string(),
            last_name: "Doe".to_string(),
            birth_date: NaiveDate::from_ymd_opt(1980, 1, 1).unwrap(),
            office_id: 1,
        }
    }

    #[test]
    fn test_valid_employee() {
        let employee = create_valid_employee();
        assert!(employee.validate().is_ok());
    }

    #[test]
    fn test_last_name_with_whitespace() {
        let mut employee = create_valid_employee();
        employee.last_name = "Lionel Messi".to_string();
        let result = employee.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Last name cannot contain whitespace");
    }

    #[test]
    fn test_born_before_1919() {
        let mut employee = create_valid_employee();
        employee.birth_date = NaiveDate::from_ymd_opt(1918, 1, 1).unwrap();
        let result = employee.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Born pre versaille treaty");
    }

    #[test]
    fn test_born_after_2007() {
        let mut employee = create_valid_employee();
        employee.birth_date = NaiveDate::from_ymd_opt(2008, 1, 1).unwrap();
        let result = employee.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Born post rust");
    }

    #[test]
    fn test_empty_last_name() {
        let mut employee = create_valid_employee();
        employee.last_name = "   ".to_string();
        let result = employee.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Last name cannot be empty");
    }
    #[test]
    fn test_empty_first_name() {
        let mut employee = create_valid_employee();
        employee.first_name = "   ".to_string();
        let result = employee.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "First name cannot be empty");
    }
}