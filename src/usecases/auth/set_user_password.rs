use sea_orm::prelude::*;
use std::sync::Arc;
use utoipa::ToSchema;

use crate::{
    adapters::postgres::entities::uroboros_user_auth_data_pg,
    apps::server::state::UroborosOauthState,
    domain::password::{generate_salt, salt_password},
};

use super::get_user_auth_data::{get_user_auth_data_by_user_id, GetUserAuthDataByUserIdOptions};

#[derive(Debug, Default, ToSchema)]
pub struct SetUserPasswordByAuthDataIdOptions {
    pub id: String,
    pub password: String,
}

pub async fn set_user_password_by_auth_data_id(
    state: Arc<UroborosOauthState>,
    options: SetUserPasswordByAuthDataIdOptions,
) -> Option<uroboros_user_auth_data_pg::Model> {
    let pg = &state.postgres;

    let salt = generate_salt();
    let pass = salt_password(&options.password, &salt);

    let id = Uuid::parse_str(&options.id).unwrap_or_default();

    let auth_data_to_edit = uroboros_user_auth_data_pg::ActiveModel {
        id: sea_orm::ActiveValue::Set(id),
        salt: sea_orm::ActiveValue::Set(salt),
        pass: sea_orm::ActiveValue::Set(pass),
        ..Default::default()
    };

    println!("auth_data_to_edit {:?}", auth_data_to_edit);

    match auth_data_to_edit.update(pg).await {
        Err(err) => {
            println!("set_user_password_by_auth_data_id {:?}", err);
            None
        }
        Ok(user_auth_data) => {
            println!("ok");
            Some(user_auth_data)
        }
    }
}

#[derive(Debug, Default, ToSchema)]
pub struct SetUserPasswordByUserIdOptions {
    user_id: String,
    password: String,
}

pub async fn set_user_password_by_user_id(
    state: Arc<UroborosOauthState>,
    options: SetUserPasswordByUserIdOptions,
) -> Option<uroboros_user_auth_data_pg::Model> {
    // let pg = &state.postgres;

    let optional_user_auth_data = get_user_auth_data_by_user_id(
        state.clone(),
        GetUserAuthDataByUserIdOptions {
            user_id: options.user_id,
        },
    )
    .await;

    match optional_user_auth_data {
        None => None,
        Some(user_auth_data) => {
            set_user_password_by_auth_data_id(
                state.clone(),
                SetUserPasswordByAuthDataIdOptions {
                    id: user_auth_data.id.to_string(),
                    password: options.password,
                },
            )
            .await
        }
    }
}
