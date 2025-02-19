use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::{
    apps::server::{
        dtos::{
            request::auth::SetUserPasswordRequestBody,
            response::{auth::TokenResponse, map::map_entity_result_to_response_tuple},
        },
        state::UroborosOauthState,
    },
    ports::dtos::core::responses::OkResponse,
    usecases::auth::{
        set_user_password::{set_user_password_by_user_id, SetUserPasswordByUserIdOptions},
        sign_in::{sign_in_by_login, SignInByLoginOptions},
    },
};

#[utoipa::path(
    post,
    path = "/auth/sign-in",
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

#[utoipa::path(
    post,
    path = "/auth/password",
    responses(
        (status = 200, description = "Set user password")
    )
)]
pub async fn set_user_password_route(
    State(state): State<Arc<UroborosOauthState>>,
    Json(body): Json<SetUserPasswordRequestBody>,
) -> impl IntoResponse {
    let user_id = body.user_id.parse().unwrap();

    let option = set_user_password_by_user_id(
        state.clone(),
        SetUserPasswordByUserIdOptions {
            user_id,
            password: body.password,
            reset: false,
        },
    )
    .await;

    map_entity_result_to_response_tuple(
        StatusCode::OK,
        match option {
            None => Ok(OkResponse { ok: false }),
            Some(_) => Ok(OkResponse { ok: true }),
        },
    )
}
