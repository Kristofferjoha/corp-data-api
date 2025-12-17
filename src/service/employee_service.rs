use crate::entity::employee::Employee;
use crate::repository::employee_repository::EmployeeRepository;
use anyhow::{anyhow};
use crate::utils::Validate;

#[derive(Clone)]
pub struct EmployeeService {
    repo: EmployeeRepository,
}

impl EmployeeService {
    pub fn new(repo: EmployeeRepository) -> Self {
        Self { repo }
    }
}