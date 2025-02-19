use std::sync::Arc;

use anyhow::Context;
use chrono::Utc;
use fake::Fake as _;

use crate::{
    apps::server::state::UroborosOauthState,
    domain::uroboros_user::uroboros_role::UroborosUserRole,
    usecases::{
        auth::{
            add_user_auth_data::AddUserAuthDataOptions,
            add_user_with_auth_data::{add_user_with_auth_data, AddUserWithAuthDataOptions},
            sign_in::{sign_in_by_login, SignInByLoginOptions},
        },
        uroboros_user::add_user::AddUserOptions,
    },
};

pub const TEST_USER_PASSWORD: &str = "eLgt%3q=_GZ.5WczFd;@j6n-#!k[Ax2)<&QEaNs/XP^JV$4b";

pub async fn get_test_client_token_by_role(
    state: Arc<UroborosOauthState>,
    role: UroborosUserRole,
) -> anyhow::Result<String> {
    let login = fake::faker::internet::pt_br::Username().fake::<String>()
        + Utc::now().timestamp().to_string().as_str();

    add_user_with_auth_data(
        state.clone(),
        AddUserWithAuthDataOptions {
            auth_data: AddUserAuthDataOptions {
                login: login.clone(),
                pass: Some(TEST_USER_PASSWORD.to_string()),
                ..Default::default()
            },
            user: AddUserOptions {
                first_name: fake::faker::name::pt_br::FirstName().fake(),
                last_name: fake::faker::name::pt_br::LastName().fake(),
                role,
                ..Default::default()
            },
        },
    )
    .await
    .context("Cannot add test user")?;

    let (token, _) = sign_in_by_login(
        state.clone(),
        SignInByLoginOptions {
            login: login.clone(),
            password: TEST_USER_PASSWORD.to_string(),
        },
    )
    .await
    .context("Cannot login as test user")?;

    Ok(token)
}
