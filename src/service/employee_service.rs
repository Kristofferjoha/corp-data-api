use crate::entity::employee::Employee;
use crate::repository::employee_repository::EmployeeRepository;
use crate::repository::office_repository::OfficeRepository;
use anyhow::{anyhow};
use crate::utils::Validate;

#[derive(Clone)]
pub struct EmployeeService {
    repo: EmployeeRepository,
    office_repo: OfficeRepository,
}

impl EmployeeService {
    pub fn new(repo: EmployeeRepository, office_repo: OfficeRepository) -> Self {
        Self { repo, office_repo }
    }

    pub async fn add_employee(&self, employee: &Employee) -> anyhow::Result<Employee> {
        tracing::info!("Attempting to add employee with name: {} {}", employee.first_name, employee.last_name);

        employee.validate().map_err(|e| anyhow::anyhow!(e))?;

        let office = self.office_repo.get_office_by_id(employee.office_id)
            .await?
            .ok_or_else(|| anyhow!("Office with ID {} does not exist", employee.office_id))?;

        let current_employee_nr = self.repo.current_employee_nr_by_office_id(employee.office_id).await?;

        if current_employee_nr >= office.max_occupancy as i64 {
            return Err(anyhow!(
                "Office {} is at full capacity: {}/{} employees", 
                office.name, 
                office.max_occupancy,
                office.max_occupancy
            ));
        }
        self.repo.create_employee(employee).await
    }

    pub async fn find_employee_by_id(&self, id: i32) -> anyhow::Result<Option<Employee>> {
        tracing::info!("Attempting to find employee with id: {}", id);
        self.repo.get_employee_by_id(id).await
    }

    pub async fn list_all_employees(&self) -> anyhow::Result<Vec<Employee>> {
        tracing::info!("Listing all employees");
        self.repo.get_all_employees().await
    }

    pub async fn list_employees_by_office_id(&self, office_id: i32) -> anyhow::Result<Vec<Employee>> {
        tracing::info!("Listing employees for office id: {}", office_id);

        let office_id_exist = self.office_repo.get_office_by_id(office_id).await?;
        if office_id_exist.is_none() {
            return Err(anyhow::anyhow!("Office with ID {} does not exist", office_id));
        }

        self.repo.get_employees_by_office_id(office_id).await
    }

    pub async fn update_employee(&self, id: i32, employee: &Employee) -> anyhow::Result<Employee> {
        tracing::info!("Attempting to update employee with id: {}", id);

        employee.validate().map_err(|e| anyhow::anyhow!(e))?;

        let office = self.office_repo.get_office_by_id(employee.office_id)
            .await?
            .ok_or_else(|| anyhow!("Office with ID {} does not exist", employee.office_id))?;

        let current_employee_nr = self.repo.current_employee_nr_by_office_id(employee.office_id).await?;

        if current_employee_nr >= office.max_occupancy as i64 {
            return Err(anyhow!(
                "Office {} is at full capacity: {}/{} employees", 
                office.name, 
                office.max_occupancy,
                office.max_occupancy
            ));
        }

        self.repo.update_employee_by_id(id, employee).await
    }

    pub async fn remove_employee(&self, id: i32) -> anyhow::Result<bool> {
        tracing::info!("Deleting employee id: {}", id);
        let rows = self.repo.delete_employee(id).await?;
        Ok(rows > 0)
    }
}