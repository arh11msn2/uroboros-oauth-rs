use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};
use sea_orm::{entity::prelude::*, ActiveValue};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{
    adapters::postgres::entities::uroboros_user_pg::{self},
    apps::server::state::UroborosOauthState,
    domain::{jwt::JwtClaims, uroboros_user::uroboros_role::UroborosUserRole},
    ports::dtos::core::responses::OkResponse,
    usecases::{
        auth::{
            add_user_auth_data::AddUserAuthDataOptions,
            add_user_with_auth_data::{add_user_with_auth_data, AddUserWithAuthDataOptions},
        },
        uroboros_user::add_user::{self, AddUserOptions},
    },
};

#[utoipa::path(
    get,
    path = "/user",
    responses(
        (status = 200, description = "Get users")
    ),
    security(
        ("token" = [])
    )
)]
pub async fn get_uroborus_users_list_route(
    State(state): State<Arc<UroborosOauthState>>,
    jwt: JwtClaims,
) -> (StatusCode, Json<Vec<uroboros_user_pg::Model>>) {
    let users = uroboros_user_pg::Entity::find()
        .all(&Arc::clone(&state).postgres)
        .await
        .unwrap();
    println!("USERS {:?}", users);
    println!("jwt {:?}", jwt);
    (StatusCode::OK, Json(users))
}

#[utoipa::path(
    post,
    path = "/user",
    responses(
        (status = 200, description = "Add user")
    ),
    security(("token" = []))
)]
pub async fn add_uroborus_user_route(
    State(state): State<Arc<UroborosOauthState>>,
    Json(user_to_add): Json<UserToAdd>,
) -> (StatusCode, Json<OkResponse>) {
    let optional_user_and_auth_data = add_user_with_auth_data(
        state.clone(),
        AddUserWithAuthDataOptions {
            user: AddUserOptions {
                role: user_to_add.role,
                first_name: user_to_add.first_name,
                last_name: user_to_add.last_name,
                patronymick: user_to_add.patronymick,
            },
            auth_data: AddUserAuthDataOptions {
                user_id: "".to_string(),
                login: user_to_add.login,
                pass: "123qwe".to_string(),
                ..Default::default()
            },
        },
    )
    .await;

    (
        StatusCode::CREATED,
        Json(OkResponse {
            ok: optional_user_and_auth_data.is_some(),
        }),
    )
}

#[derive(Deserialize, ToSchema)]
pub struct UserToAdd {
    login: String,
    role: UroborosUserRole,
    first_name: String,
    last_name: String,
    patronymick: Option<String>,
}

#[derive(Serialize)]
pub struct User {
    id: i32,
    name: String,
}
