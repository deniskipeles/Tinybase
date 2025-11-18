use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{post},
    Json, Router,
};
use serde::Serialize;
use std::sync::Arc;
use tinybase_core::models::{Collection, Record};
use tinybase_core::Db;

pub type AppState = Arc<dyn Db>;

#[derive(Serialize)]
pub struct CollectionResponse {
    id: i64,
    name: String,
}

#[derive(Serialize)]
pub struct RecordResponse {
    id: i64,
    data: serde_json::Value,
}

#[derive(Serialize)]
struct ProblemDetail {
    error: String,
    message: String,
    details: Option<serde_json::Value>,
    status: u16,
}

pub enum AppError {
    LibsqlError(libsql::Error),
    JsonError(String),
    UnknownError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, problem) = match self {
            AppError::LibsqlError(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ProblemDetail {
                    error: "database_error".to_string(),
                    message: "A database error occurred.".to_string(),
                    details: Some(serde_json::json!({ "db_error": e.to_string() })),
                    status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                },
            ),
            AppError::JsonError(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ProblemDetail {
                    error: "serialization_error".to_string(),
                    message: "Failed to serialize data.".to_string(),
                    details: Some(serde_json::json!({ "json_error": e })),
                    status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                },
            ),
            AppError::UnknownError(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ProblemDetail {
                    error: "unknown_error".to_string(),
                    message: "An unknown error occurred.".to_string(),
                    details: Some(serde_json::json!({ "error": e })),
                    status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                },
            ),
        };

        (status, Json(problem)).into_response()
    }
}

impl From<libsql::Error> for AppError {
    fn from(e: libsql::Error) -> Self {
        AppError::LibsqlError(e)
    }
}

pub fn app_router(db: AppState) -> Router {
    Router::new()
        .nest(
            "/api/v1",
            Router::new()
                .route("/collections", post(create_collection))
                .route("/collections/:id/records", post(create_record)),
        )
        .with_state(db)
}

async fn create_collection(
    State(db): State<AppState>,
    Json(payload): Json<Collection>,
) -> Result<(StatusCode, Json<CollectionResponse>), AppError> {
    let id = db.create_collection(&payload.name).await?;
    Ok((
        StatusCode::CREATED,
        Json(CollectionResponse {
            id,
            name: payload.name,
        }),
    ))
}

async fn create_record(
    State(db): State<AppState>,
    Path(id): Path<i64>,
    Json(payload): Json<Record>,
) -> Result<(StatusCode, Json<RecordResponse>), AppError> {
    let record_id = db.create_record(id, &payload.data).await.map_err(|e| {
        if let Some(e) = e.downcast_ref::<serde_json::Error>() {
            AppError::JsonError(e.to_string())
        } else if let Ok(e) = e.downcast::<libsql::Error>() {
            AppError::LibsqlError(*e)
        } else {
            AppError::UnknownError("An unknown error occurred".to_string())
        }
    })?;
    Ok((
        StatusCode::CREATED,
        Json(RecordResponse {
            id: record_id,
            data: payload.data,
        }),
    ))
}
