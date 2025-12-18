use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
    response::IntoResponse,
    http::StatusCode,
};
use std::sync::Arc;
use crate::service::office_service::OfficeService;
use crate::entity::office::Office;
use crate::dto::office_dto::{CreateOfficeRequest, OfficeResponse};

/// Creates the office API router.
///
/// Routes:
/// Create a new office: POST /offices
/// Get office by ID: GET /offices/{id}
/// List all offices: GET /offices
/// Update office by ID: PUT /offices/{id}
/// Delete office by ID: DELETE /offices/{id}

pub fn create_router(service: Arc<OfficeService>) -> Router {
    Router::new()
        .route("/offices", post(create_office).get(list_all_offices))
        .route("/offices/{id}", get(get_office_by_id). put(update_office).delete(delete_office))
        .with_state(service)
}

/// Creates office
/// Expects JSON body with office data
/// Success returns 201 Created with office data
/// Failure returns 400 Bad Request with error message
#[utoipa::path(
    post,
    path = "/offices",
    request_body = CreateOfficeRequest,
    responses(
        (status = 201, description = "Office created successfully", body = OfficeResponse),
        (status = 400, description = "Bad request")
    )
)]
pub async fn create_office(
    State(service): State<Arc<OfficeService>>,
    Json(req): Json<CreateOfficeRequest>,
) -> impl IntoResponse {
    tracing::info!("Received request to create office: {}", req.name);
    let office = Office::from_create_request(req);
    
    match service.add_office(&office).await {
        Ok(new_office) => {
            tracing::info!("Successfully created office with ID: {:?}", new_office.id.unwrap());
            (StatusCode::CREATED, Json(new_office.to_response())).into_response()
        },
        Err(e) => {
            tracing::warn!("Failed to process office creation: {}", e);
            (StatusCode::BAD_REQUEST, Json(e.to_string())).into_response()
        }
    }
}

/// Retrieves office by ID
/// Expects office ID as a path parameter
/// Success returns 200 OK with office data
/// Failure returns 404 Not Found or 500 Internal Server Error
#[utoipa::path(
    get,
    path = "/offices/{id}",
    params(
        ("id" = i32, Path, description = "Office ID")
    ),
    responses(
        (status = 200, description = "Office found", body = OfficeResponse),
        (status = 404, description = "Office not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_office_by_id(
    State(service): State<Arc<OfficeService>>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    tracing::info!("Received request to get office by id: {}", id);
    match service.find_office_by_id(id).await {
        Ok(Some(office)) => {
            tracing::info!("Office with id {} found", id);
            Json(office.to_response()).into_response()
        }
        Ok(None) => {
            tracing::warn!("Office with id {} not found", id);
            (StatusCode::NOT_FOUND, "Office not found").into_response()
        }
        Err(e) => {
            tracing::error!("Error finding office {}: {}", id, e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
        }
    }
}

/// Lists all offices
/// No parameters required
/// Success returns 200 OK with a list of offices
/// Failure returns 500 Internal Server Error
#[utoipa::path(
    get,
    path = "/offices",
    responses(
        (status = 200, description = "List of all offices", body = Vec<OfficeResponse>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn list_all_offices(
    State(service): State<Arc<OfficeService>>,
) -> impl IntoResponse {
    tracing::info!("Received request to list all offices");
    match service.list_all_offices().await {
        Ok(offices) => {
            let responses: Vec<_> = offices.into_iter().map(|o| o.to_response()).collect();
            tracing::info!("Found a total of {} offices", responses.len());
            Json(responses).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to list offices: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
        },
    }
}

/// Updates office by ID
/// Expects office ID as a path parameter and JSON body with updated data
/// Success returns 200 OK with updated office data
/// Failure returns 400 Bad Request
#[utoipa::path(
    put,
    path = "/offices/{id}",
    params(
        ("id" = i32, Path, description = "Office ID")
    ),
    request_body = CreateOfficeRequest,
    responses(
        (status = 200, description = "Office updated successfully", body = OfficeResponse),
        (status = 400, description = "Bad request")
    )
)]
pub async fn update_office(
    State(service): State<Arc<OfficeService>>,
    Path(id): Path<i32>,
    Json(req): Json<CreateOfficeRequest>,
) -> impl IntoResponse {
    tracing::info!("Received request to update office with id: {}", id);
    let office = Office::from_create_request(req);

    match service.update_office(id, &office).await {
        Ok(updated) => {
            tracing::info!("Sucessfully updated office with id: {}", id);
            (StatusCode::OK, Json(updated.to_response())).into_response()
        },
        Err(e) => {
            tracing::warn!("Failed to update office ID {}: {}", id, e);
            (StatusCode::BAD_REQUEST, e.to_string()).into_response()
        },
    }
}

/// Deletes office by ID
/// Expects office ID as a path parameter
/// Success returns 204 No Content
/// Failure returns 404 Not Found or 500 Internal Server Error
#[utoipa::path(
    delete,
    path = "/offices/{id}",
    params(
        ("id" = i32, Path, description = "Office ID")
    ),
    responses(
        (status = 204, description = "Office deleted successfully"),
        (status = 404, description = "Office not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn delete_office(
    State(service): State<Arc<OfficeService>>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    tracing::info!("Received request to delete office with id: {}", id);
    match service.remove_office(id).await {
        Ok(true) => {
            tracing::info!("Successfully deleted office with id: {}", id);
            StatusCode::NO_CONTENT.into_response()
        },
        Ok(false) => {
            tracing::warn!("Failed as office not found for office with id: {}", id);
            (StatusCode::NOT_FOUND, "Office not found").into_response()
        }
        Err(e) => {
            tracing::error!("Error to delete ID {}: {}", id, e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
        }
    }
}