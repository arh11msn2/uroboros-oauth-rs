use std::sync::Arc;
use utoipa::ToSchema;

use crate::{
    adapters::postgres::entities::{uroboros_user_auth_data_pg, uroboros_user_pg},
    apps::server::state::UroborosOauthState,
    usecases::uroboros_user::add_user::{add_user, AddUserOptions},
};

use super::add_user_auth_data::{add_user_auth_data, AddUserAuthDataOptions};

#[derive(Debug, Default, ToSchema)]
pub struct AddUserWithAuthDataOptions {
    pub user: AddUserOptions,
    pub auth_data: AddUserAuthDataOptions,
}

pub async fn add_user_with_auth_data(
    state: Arc<UroborosOauthState>,
    options: AddUserWithAuthDataOptions,
) -> Option<(uroboros_user_pg::Model, uroboros_user_auth_data_pg::Model)> {
    println!("add_user_with_auth_data {:?}", options);
    let user = add_user(state.clone(), options.user).await.ok()?;

    let mut auth_data_options = options.auth_data.clone();
    auth_data_options.user_id = user.id.to_string();

    match add_user_auth_data(state.clone(), auth_data_options).await {
        None => None,
        Some(auth_data) => Some((user, auth_data)),
    }
}
