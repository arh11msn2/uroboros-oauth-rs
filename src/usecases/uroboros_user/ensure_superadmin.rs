use std::sync::Arc;

use crate::{
    apps::server::state::UroborosOauthState,
    domain::{
        result::{UroborosError, UroborosErrorKind, UroborosResult},
        uroboros_user::uroboros_role::UroborosUserRole,
    },
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

pub async fn ensure_superadmin(state: Arc<UroborosOauthState>) -> UroborosResult<()> {
    let superadmin_options = &state.superadmin_options.clone().ok_or(UroborosError {
        kind: UroborosErrorKind::NotFound,
        message: "Default superadmin config not found".to_string(),
    })?;

    let optional_user_and_auth_data = get_user_and_auth_data_by_login(
        state.clone(),
        GetUserAndAuthDataByLoginOptions {
            login: superadmin_options.login.clone(),
        },
    )
    .await;

    match optional_user_and_auth_data {
        None => {
            add_user_with_auth_data(
                state.clone(),
                AddUserWithAuthDataOptions {
                    user: AddUserOptions {
                        role: UroborosUserRole::Superadmin,
                        first_name: String::from("Uroboros"),
                        last_name: String::from("Superadmin"),
                        ..Default::default()
                    },
                    auth_data: AddUserAuthDataOptions {
                        user_id: String::new(),
                        login: superadmin_options.login.clone(),
                        pass: superadmin_options.password.clone(),
                        ..Default::default()
                    },
                },
            )
            .await;
        }
        Some((_, superadmin_auth_data)) => {
            println!("superadmin_auth_data {:?}", superadmin_auth_data);
            set_user_password_by_auth_data_id(
                state.clone(),
                SetUserPasswordByAuthDataIdOptions {
                    id: superadmin_auth_data.id.to_string(),
                    password: superadmin_options.password.clone(),
                },
            )
            .await;
        }
    };

    Ok(())
}
