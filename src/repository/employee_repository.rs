use crate::entity::employee::Employee;
use sqlx::PgPool;


#[derive(Clone)]
pub struct EmployeeRepository {
    pool: PgPool,
}
impl EmployeeRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_employee(&self, employee: &Employee) -> anyhow::Result<Employee> {
        let created = sqlx::query_as!(
            Employee,
            "INSERT INTO employees (first_name, last_name, birth_date, office_id) VALUES ($1, $2, $3, $4) RETURNING id, first_name, last_name, birth_date, office_id",
            employee.first_name,
            employee.last_name,
            employee.birth_date,
            employee.office_id
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(created)
    }

    pub async fn current_employee_nr_by_office_id(&self, office_id: i32) -> anyhow::Result<i64> {
        let count = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM employees WHERE office_id = $1",
            office_id
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(count.unwrap_or(0))
    }

    pub async fn get_employee_by_id(&self, id: i32) -> anyhow::Result<Option<Employee>> {
        let employee = sqlx::query_as!(
            Employee,
            "SELECT id, first_name, last_name, birth_date, office_id FROM employees WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(employee)
    }

    pub async fn get_all_employees(&self) -> anyhow::Result<Vec<Employee>> {
        let employees = sqlx::query_as!(
            Employee,
            "SELECT id, first_name, last_name, birth_date, office_id FROM employees"
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(employees)
    }

    pub async fn delete_employee(&self, id: i32) -> anyhow::Result<u64> {
        let result = sqlx::query!("DELETE FROM employees WHERE id = $1",id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected())
    }

}