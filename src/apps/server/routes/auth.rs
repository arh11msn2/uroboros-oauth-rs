use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::{
    apps::server::state::UroborosOauthState,
    usecases::auth::sign_in::{sign_in_by_login, SignInByLoginOptions},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    pub token: Option<String>,
}

#[utoipa::path(
    post,
    path = "/sign-in",
    responses(
        (status = 200, description = "Sign in")
    )
)]
#[allow(dead_code)]
pub async fn sign_in_route(
    State(state): State<Arc<UroborosOauthState>>,
    Json(payload): Json<SignInByLoginOptions>,
) -> impl IntoResponse {
    match sign_in_by_login(state.clone(), payload).await {
        None => (StatusCode::FORBIDDEN, Json(TokenResponse { token: None })),
        Some((token, _)) => (StatusCode::OK, Json(TokenResponse { token: Some(token) })),
    }
}
