use std::sync::Arc;

use sea_orm::{prelude::*, ColumnTrait};

use crate::{
    adapters::postgres::entities::{uroboros_user_auth_data_pg, uroboros_user_pg},
    apps::server::state::UroborosOauthState,
};

pub struct GetUserAndAuthDataByLoginOptions {
    pub login: String,
}

pub async fn get_user_and_auth_data_by_login(
    state: Arc<UroborosOauthState>,
    options: GetUserAndAuthDataByLoginOptions,
) -> Option<(uroboros_user_pg::Model, uroboros_user_auth_data_pg::Model)> {
    let pg = &state.postgres;

    match uroboros_user_auth_data_pg::Entity::find()
        .filter(uroboros_user_auth_data_pg::Column::Login.eq(options.login))
        .find_also_related(uroboros_user_pg::Entity)
        .one(pg)
        .await
    {
        Err(_) => None,
        Ok(row) => match row {
            None => None,
            Some((auth_data, opt_user)) => opt_user.map(|user| (user, auth_data)),
        },
    }
}
