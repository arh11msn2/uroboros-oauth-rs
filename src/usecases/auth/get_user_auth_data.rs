use std::sync::Arc;

use sea_orm::prelude::*;
use utoipa::ToSchema;

use crate::{
    adapters::postgres::entities::uroboros_user_auth_data_pg,
    apps::server::state::UroborosOauthState,
};

#[derive(Debug, Default, ToSchema)]
pub struct GetUserAuthDataByUserIdOptions {
    pub user_id: String,
}

pub async fn get_user_auth_data_by_user_id(
    state: Arc<UroborosOauthState>,
    options: GetUserAuthDataByUserIdOptions,
) -> Option<uroboros_user_auth_data_pg::Model> {
    let pg = &state.postgres;

    match uroboros_user_auth_data_pg::Entity::find()
        .filter(uroboros_user_auth_data_pg::Column::UserId.eq(options.user_id))
        .one(pg)
        .await
    {
        Err(_) => None,
        Ok(user_auth_data) => user_auth_data,
    }
}
