use std::sync::Arc;

use sea_orm::prelude::*;

use crate::{
    adapters::postgres::entities::user_auth_data_pg, apps::server::state::UroborosOauthState,
};

#[derive(Debug, Default)]
pub struct GetUserAuthDataByUserIdOptions {
    pub user_id: Uuid,
}

pub async fn get_user_auth_data_by_user_id(
    state: Arc<UroborosOauthState>,
    options: GetUserAuthDataByUserIdOptions,
) -> Option<user_auth_data_pg::Model> {
    let result = user_auth_data_pg::Entity::find()
        .filter(user_auth_data_pg::Column::UserId.eq(options.user_id))
        .one(&state.postgres)
        .await;

    result.ok().unwrap_or_default()
}
