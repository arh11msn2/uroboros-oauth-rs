use std::sync::Arc;

use anyhow::Context;

use crate::{
    apps::server::state::UroborosOauthState,
    domain::uroboros_user::uroboros_role::UroborosUserRole,
    usecases::auth::{
        add_user_auth_data::AddUserAuthDataOptions,
        add_user_with_auth_data::{add_user_with_auth_data, AddUserWithAuthDataOptions},
        get_user_and_auth_data::{
            get_user_and_auth_data_by_login, GetUserAndAuthDataByLoginOptions,
        },
        set_user_password::{
            set_user_password_by_auth_data_id, SetUserPasswordByAuthDataIdOptions,
        },
    },
};

use super::add_user::AddUserOptions;

pub async fn ensure_default_admin(state: Arc<UroborosOauthState>) -> anyhow::Result<()> {
    let default_admin_options = &state
        .default_admin_options
        .clone()
        .context("Default admin config not found")?;

    let optional_user_and_auth_data = get_user_and_auth_data_by_login(
        state.clone(),
        GetUserAndAuthDataByLoginOptions {
            login: default_admin_options.login.clone(),
        },
    )
    .await;

    match optional_user_and_auth_data {
        None => {
            add_user_with_auth_data(
                state.clone(),
                AddUserWithAuthDataOptions {
                    user: AddUserOptions {
                        role: UroborosUserRole::Admin,
                        first_name: String::from("Uroboros"),
                        last_name: String::from("DefaultAdmin"),
                        ..Default::default()
                    },
                    auth_data: AddUserAuthDataOptions {
                        user_id: String::new(),
                        login: default_admin_options.login.clone(),
                        pass: Some(default_admin_options.password.clone()),
                        ..Default::default()
                    },
                },
            )
            .await;
        }
        Some((_, default_admin_auth_data)) => {
            set_user_password_by_auth_data_id(
                state.clone(),
                SetUserPasswordByAuthDataIdOptions {
                    id: default_admin_auth_data.id.to_string(),
                    password: default_admin_options.password.clone(),
                    reset: true,
                },
            )
            .await;
        }
    };

    Ok(())
}
