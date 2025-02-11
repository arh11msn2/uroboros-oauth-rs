use sea_orm::prelude::*;
use std::sync::Arc;
use utoipa::ToSchema;

use crate::{
    adapters::postgres::entities::uroboros_user_auth_data_pg,
    apps::server::state::UroborosOauthState,
    domain::password::{generate_salt, salt_password},
};

#[derive(Debug, Clone, Default, ToSchema)]
pub struct AddUserAuthDataOptions {
    pub user_id: String,
    pub login: String,
    pub pass: String,
    pub salt: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
}

pub async fn add_user_auth_data(
    state: Arc<UroborosOauthState>,
    options: AddUserAuthDataOptions,
) -> Option<uroboros_user_auth_data_pg::Model> {
    let salt = generate_salt();
    let pass = salt_password(&options.pass, &salt);

    println!("OKOK");
    println!("{:?}", options);
    println!("{:?}", &options.user_id);
    println!(
        "{:?}",
        Uuid::parse_str(&options.user_id).unwrap_or_default()
    );

    let auth_data_to_add = uroboros_user_auth_data_pg::ActiveModel {
        user_id: sea_orm::ActiveValue::Set(Uuid::parse_str(&options.user_id).unwrap_or_default()),
        login: sea_orm::ActiveValue::Set(options.login),
        pass: sea_orm::ActiveValue::Set(pass),
        salt: sea_orm::ActiveValue::Set(salt),
        ..Default::default()
    };

    let pg = &state.postgres;

    match auth_data_to_add.insert(pg).await {
        Err(err) => {
            println!("Error {:?}", err);
            None
        }
        Ok(auth_data) => {
            println!("AuthData {:?}", auth_data);
            Some(auth_data)
        }
    }
}
