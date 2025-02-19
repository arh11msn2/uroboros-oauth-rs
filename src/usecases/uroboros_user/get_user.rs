use anyhow::Context;
use sea_orm::prelude::*;
use std::sync::Arc;

use crate::{
    adapters::postgres::entities::user_pg, apps::server::state::UroborosOauthState,
    domain::uroboros_user::uroboros_role::UroborosUserRole,
};

#[derive(Debug, Default)]
pub struct GetUserByIdOptions {
    pub user_id: Uuid,
}

pub async fn get_user_by_id(
    state: Arc<UroborosOauthState>,
    options: GetUserByIdOptions,
) -> anyhow::Result<user_pg::Model> {
    let o_user = user_pg::Entity::find_by_id(options.user_id)
        .one(&state.postgres)
        .await?;

    o_user.context("User not found by id")
}

#[derive(Debug, Default)]
pub struct GetActorByIdOptions {
    pub actor_id: Uuid,
}

pub async fn get_actor_by_id(
    state: Arc<UroborosOauthState>,
    options: GetActorByIdOptions,
) -> anyhow::Result<user_pg::Model> {
    get_user_by_id(
        state.clone(),
        GetUserByIdOptions {
            user_id: options.actor_id,
        },
    )
    .await
}

#[derive(Debug, Default)]
pub struct GetOneUserByActorById {
    pub actor_id: Uuid,
    pub user_id: Uuid,
}

pub async fn get_one_user_by_actor_by_id(
    state: Arc<UroborosOauthState>,
    options: GetOneUserByActorById,
) -> anyhow::Result<user_pg::Model> {
    let actor = get_actor_by_id(
        state.clone(),
        GetActorByIdOptions {
            actor_id: options.actor_id,
        },
    )
    .await?;

    if actor.id == options.user_id || actor.role == UroborosUserRole::Admin {
        get_user_by_id(
            state.clone(),
            GetUserByIdOptions {
                user_id: options.user_id,
            },
        )
        .await
    } else {
        anyhow::bail!("Cannot get user: forbidden")
    }
}
