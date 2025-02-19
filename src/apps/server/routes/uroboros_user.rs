use std::sync::Arc;

use anyhow::Context;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sea_orm::entity::prelude::*;

use crate::{
    adapters::postgres::entities::user_pg::{self},
    apps::server::{
        dtos::{
            request::user::AddUserRequestBody, response::map::map_entity_result_to_response_tuple,
        },
        state::UroborosOauthState,
    },
    domain::jwt::JwtClaims,
    usecases::{
        auth::{
            add_user_auth_data::AddUserAuthDataOptions,
            add_user_with_auth_data::{add_user_with_auth_data, AddUserWithAuthDataOptions},
        },
        uroboros_user::{
            add_user::AddUserOptions,
            get_user::{
                get_one_user_by_actor_by_id, get_user_by_id, GetOneUserByActorById,
                GetUserByIdOptions,
            },
        },
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
) -> (StatusCode, Json<Vec<user_pg::Model>>) {
    let users = user_pg::Entity::find()
        .all(&Arc::clone(&state).postgres)
        .await
        .unwrap();
    println!("users {:?}", users);
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
    Json(user_to_add): Json<AddUserRequestBody>,
) -> impl IntoResponse {
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
                ..Default::default()
            },
        },
    )
    .await;

    let r_user = optional_user_and_auth_data
        .context("User not added")
        .map(|(user, _)| user);

    map_entity_result_to_response_tuple(StatusCode::CREATED, r_user)
}

#[utoipa::path(
    get,
    path = "/user/me",
    responses(
        (status = 200, description = "Get current user")
    ),
    security(("token" = []))
)]
pub async fn get_current_user_route(
    jwt: JwtClaims,
    State(state): State<Arc<UroborosOauthState>>,
) -> impl IntoResponse {
    let user_id = jwt.uid.parse().unwrap_or_default();

    let r_user = get_user_by_id(state.clone(), GetUserByIdOptions { user_id }).await;

    map_entity_result_to_response_tuple(StatusCode::OK, r_user)
}

#[utoipa::path(
    get,
    path = "/user/{user_id}",
    params(("user_id" = String, Path)),
    responses(
        (status = 200, description = "Get one user by id")
    ),
    security(("token" = []))
)]
pub async fn get_one_user_by_id_route(
    jwt: JwtClaims,
    State(state): State<Arc<UroborosOauthState>>,
    Path(user_id): Path<Uuid>,
) -> impl IntoResponse {
    let r_user = get_one_user_by_actor_by_id(
        state,
        GetOneUserByActorById {
            actor_id: jwt.uid.parse().unwrap(), // TODO: Change unwrap to result handling
            user_id,
        },
    )
    .await;

    map_entity_result_to_response_tuple(StatusCode::OK, r_user)
}
