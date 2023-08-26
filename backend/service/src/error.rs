use axum::{
    extract::Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

pub type SvcErr = ServiceError;
pub type SvcResult<T, E = ServiceError> = Result<T, E>;

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ServiceError {
    UsernameTaken,
    InvalidLogin,
    UnavailableLogin,

    NoSession,

    Database,
}

impl IntoResponse for ServiceError {
    fn into_response(self) -> Response {
        use ServiceError::*;
        let code = match self {
            UsernameTaken => StatusCode::CONFLICT,
            InvalidLogin => StatusCode::UNAUTHORIZED,
            UnavailableLogin => StatusCode::BAD_REQUEST,

            NoSession => StatusCode::UNAUTHORIZED,

            Database => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (code, Json(self)).into_response()
    }
}

impl From<sea_orm::DbErr> for ServiceError {
    fn from(_value: sea_orm::DbErr) -> Self {
        ServiceError::Database
    }
}
