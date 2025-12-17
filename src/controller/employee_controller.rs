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
    Router::new()
        .route("/employees", post(create_employee).get(list_all_employees))
        .route("/employees/{id}", get(get_employee_by_id).delete(delete_employee))
        .route("/employees/office/{office_id}", get(list_employees_by_office_id))
        .with_state(service)
}

// create employee C
async fn create_employee(
    State(service): State<Arc<EmployeeService>>,
    Json(req): Json<CreateEmployeeRequest>,
) -> impl IntoResponse {
    tracing::info!("Received request to create employee: {} {}", req.first_name, req.last_name);
    let employee = Employee::from_create_request(req);
    
    match service.add_employee(&employee).await {
        Ok(new_employee) => {
            tracing::info!("Successfully created employee with ID: {:?}", new_employee.id.unwrap());
            (StatusCode::CREATED, Json(new_employee.to_response())).into_response()
        },
        Err(e) => {
            tracing::warn!("Failed to process employee creation: {}", e);
            (StatusCode::BAD_REQUEST, Json(e.to_string())).into_response()
        }
    }
}
// get employee by id R
async fn get_employee_by_id(
    State(service): State<Arc<EmployeeService>>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    tracing::info!("Received request to get employee by id: {}", id);
    match service.find_employee_by_id(id).await {
        Ok(Some(employee)) => {
            tracing::info!("Employee with id {} found", id);
            Json(employee.to_response()).into_response()
        }
        Ok(None) => {
            tracing::warn!("Employee with id {} not found", id);
            (StatusCode::NOT_FOUND, "Employee not found").into_response()
        }
        Err(e) => {
            tracing::error!("Error finding employee {}: {}", id, e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
        }
    }
}
// get all employees R
async fn list_all_employees(
    State(service): State<Arc<EmployeeService>>,
) -> impl IntoResponse {
    tracing::info!("Received request to list all employees");
    match service.list_all_employees().await {
        Ok(employees) => {
            tracing::info!("Successfully retrieved {} employees", employees.len());
            let response: Vec<_> = employees.into_iter().map(|e| e.to_response()).collect();
            Json(response).into_response()
        },
        Err(e) => {
            tracing::error!("Error listing employees: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
        }
    }
}
// get all employees office R
async fn list_employees_by_office_id(
    State(service): State<Arc<EmployeeService>>,
    Path(office_id): Path<i32>,
) -> impl IntoResponse {
    tracing::info!("Received request to list employees for office id: {}", office_id);

    match service.list_employees_by_office_id(office_id).await {
        Ok(employees) => {
            tracing::info!("Successfully retrieved {} employees for office id {}", employees.len(), office_id);
            let response: Vec<_> = employees.into_iter().map(|e| e.to_response()).collect();
            Json(response).into_response()
        },
        Err(e) => {
            let error_msg = e.to_string();
            if error_msg.contains("does not exist") {
                tracing::warn!("Office lookup failed: {}", error_msg);
                (StatusCode::NOT_FOUND, error_msg).into_response()
            } else {
                tracing::error!("Database error listing employees: {}", error_msg);
                (StatusCode::INTERNAL_SERVER_ERROR, error_msg).into_response()
            }
        }
    }
}
// update employee U

// delete employee D
async fn delete_employee(
    State(service): State<Arc<EmployeeService>>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    tracing::info!("Received request to delete employee with id: {}", id);
    match service.remove_employee(id).await {
        Ok(true) => {
            tracing::info!("Successfully deleted employee with id: {}", id);
            (StatusCode::NO_CONTENT).into_response()
        },
        Ok(false) => {
            tracing::warn!("Failed as employee not found for employee with id: {}", id);
            (StatusCode::NOT_FOUND, "Employee not found").into_response()
        },
        Err(e) => {
            tracing::error!("Error deleting employee {}: {}", id, e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
        }
    }
}