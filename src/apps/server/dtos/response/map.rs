use axum::{http::StatusCode, Json};

pub fn map_entity_result_to_response_tuple<T>(
    code: StatusCode,
    result: anyhow::Result<T>,
) -> (StatusCode, Json<Option<T>>) {
    match result {
        Err(_err) => (StatusCode::INTERNAL_SERVER_ERROR, Json(None)),
        Ok(it) => (code, Json(Some(it))),
    }
}
