use sea_orm::{prelude::*, Set};
use std::sync::Arc;
use utoipa::ToSchema;

use crate::{
    adapters::postgres::entities::user_pg,
    apps::server::state::UroborosOauthState,
    domain::{
        result::{UroborosError, UroborosErrorKind, UroborosResult},
        uroboros_user::uroboros_role::UroborosUserRole,
    },
};

#[derive(Debug, Default, ToSchema)]
pub struct AddUserOptions {
    pub role: UroborosUserRole,
    pub first_name: String,
    pub last_name: String,
    pub patronymick: Option<String>,
}

pub async fn add_user(
    state: Arc<UroborosOauthState>,
    options: AddUserOptions,
) -> UroborosResult<user_pg::Model> {
    let mut user_to_add = user_pg::ActiveModel {
        role: Set(options.role),
        first_name: Set(options.first_name.to_string()),
        last_name: Set(options.last_name.to_string()),
        ..Default::default()
    };

    if options.patronymick.is_some() {
        user_to_add.patronymick = sea_orm::ActiveValue::Set(options.patronymick);
    }

    user_to_add.insert(&state.postgres).await.map_err(|err| {
        println!("add_user.error: {:?}", err);
        UroborosError {
            kind: UroborosErrorKind::CannotAdd,
            message: "Cannot add user".to_string(),
        }
    })
}
