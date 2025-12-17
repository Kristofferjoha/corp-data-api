use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
    response::IntoResponse,
    http::StatusCode,
};
use std::sync::Arc;
use crate::service::employee_service::EmployeeService;
use crate::entity::employee::Employee;
use crate::dto::employee_dto::CreateEmployeeRequest;

pub fn create_router(service: Arc<EmployeeService>) -> Router {
    // create employee C
    // get employee by id R
    // get all employees R
    // get all employees office R
    // get number of employees office R, bruges til max occ
    // update employee U
    // delete employee D
}