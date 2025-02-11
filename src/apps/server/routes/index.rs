use axum::{http::StatusCode, response::IntoResponse};

#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, description = "Send a salute from Uroborus Oauth Server")
    )
)]
pub async fn get_index_route() -> impl IntoResponse {
    (StatusCode::OK, "Hello, Uroborus Oauth Server")
}
