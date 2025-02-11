use sea_orm::{prelude::*, Set};
use std::sync::Arc;

use crate::{
    adapters::postgres::entities::uroboros_user_pg,
    apps::server::state::UroborosOauthState,
    domain::result::{UroborosError, UroborosErrorKind, UroborosResult},
};

#[derive(Debug, Default)]
pub struct GetUserByIdOptions {
    pub user_id: Uuid,
}

pub async fn get_user_by_id(
    state: Arc<UroborosOauthState>,
    options: GetUserByIdOptions,
) -> UroborosResult<uroboros_user_pg::Model> {
    let user = uroboros_user_pg::Entity::find_by_id(options.user_id)
        .one(&state.postgres)
        .await
        .map_err(|err| {
            println!("get_user_by_id.error: {:?}", err);
            UroborosError {
                kind: UroborosErrorKind::NotFound,
                message: format!("User not found"),
            }
        })?;

    user.ok_or(UroborosError {
        kind: UroborosErrorKind::NotFound,
        message: format!("User not found"),
    })
}

#[derive(Debug, Default)]
pub struct GetActorByIdOptions {
    pub actor_id: Uuid,
}

pub async fn get_actor_by_id(
    state: Arc<UroborosOauthState>,
    options: GetActorByIdOptions,
) -> UroborosResult<uroboros_user_pg::Model> {
    get_user_by_id(
        state.clone(),
        GetUserByIdOptions {
            user_id: options.actor_id,
        },
    )
    .await
    .map_err(|_| UroborosError {
        kind: UroborosErrorKind::Unauthorized,
        message: format!("Unauthorized"),
    })
}
