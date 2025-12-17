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

}