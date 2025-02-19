use std::sync::Arc;

use ::chrono::TimeDelta;
use sea_orm::sqlx::types::chrono::Utc;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{
    adapters::postgres::entities::user_pg,
    apps::server::state::UroborosOauthState,
    domain::{
        jwt::{encode_jwt, JwtClaims},
        password::salt_password,
    },
    usecases::auth::get_user_and_auth_data::{
        get_user_and_auth_data_by_login, GetUserAndAuthDataByLoginOptions,
    },
};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SignInByLoginOptions {
    pub login: String,
    pub password: String,
}

pub async fn sign_in_by_login(
    state: Arc<UroborosOauthState>,
    options: SignInByLoginOptions,
) -> Option<(String, user_pg::Model)> {
    let optional_user_by_login = get_user_and_auth_data_by_login(
        state.clone(),
        GetUserAndAuthDataByLoginOptions {
            login: options.login.clone(),
        },
    )
    .await;

    match optional_user_by_login {
        None => None,
        Some((user, auth_data)) => {
            let password = salt_password(&options.password, &auth_data.salt);
            let expected_pass = auth_data.pass;

            if Some(password) == expected_pass {
                match encode_jwt(
                    JwtClaims {
                        uid: user.id.to_string(),
                        exp: (Utc::now() + TimeDelta::days(3)).timestamp() as usize,
                    },
                    "secret11",
                ) {
                    Err(err) => {
                        println!("encode_jwt.error: {:?}", err);
                        None
                    }
                    Ok(token) => Some((token, user)),
                }
            } else {
                None
            }
        }
    }
}
