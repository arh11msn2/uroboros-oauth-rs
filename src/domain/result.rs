use core::result;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

/// An error that can occur when encoding/decoding JWTs
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct UroborosError {
    pub kind: UroborosErrorKind,
    pub message: String,
}

impl Default for UroborosError {
    fn default() -> Self {
        Self {
            kind: UroborosErrorKind::Unknown,
            message: "Unknown".to_string(),
        }
    }
}

impl IntoResponse for UroborosError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.message).into_response()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub enum UroborosErrorKind {
    CannotAdd,
    CannotEdit,
    CannotGet,
    NotFound,
    Unauthorized,
    Unknown,
}

pub type UroborosResult<T> = result::Result<T, UroborosError>;

impl From<UroborosError> for anyhow::Error {
    fn from(value: UroborosError) -> Self {
        Self::msg(format!("{:?}", value))
    }
}

/*
impl <T> IntoResponse for result::Result<T, UroborosError>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        match self {
            Ok(t) => Json(t).into_response(),
            Err(err) => err.into_response(),
        }
    }
}
 */
