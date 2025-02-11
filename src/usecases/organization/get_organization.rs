use sea_orm::{prelude::*, QuerySelect};
use std::sync::Arc;

use crate::{
    adapters::postgres::entities::{organization_member_pg, organization_pg},
    apps::server::state::UroborosOauthState,
    domain::result::{UroborosError, UroborosErrorKind, UroborosResult},
    usecases::uroboros_user::get_user::{get_actor_by_id, GetActorByIdOptions},
};

#[derive(Debug, Default)]
pub struct GetOrganizationPageByUserOptions {
    pub actor_id: Uuid,
    pub page: u64,
}

pub async fn get_organization_page_by_user(
    state: Arc<UroborosOauthState>,
    options: GetOrganizationPageByUserOptions,
) -> UroborosResult<Vec<organization_pg::Model>> {
    println!("options {:?}", options);
    let actor = get_actor_by_id(
        state.clone(),
        GetActorByIdOptions {
            actor_id: options.actor_id,
        },
    )
    .await?;

    let limit: u64 = 100;
    let offset: u64 = (options.page - 1) * limit;

    organization_pg::Entity::find()
        .limit(limit)
        .offset(offset)
        .all(&state.postgres)
        .await
        .map_err(|err| {
            println!("err {:?}", err);
            UroborosError {
                kind: UroborosErrorKind::CannotGet,
                message: "Cannot get organizations page".to_string(),
            }
        })
}

#[derive(Debug, Default)]
pub struct GetOneOrganizationByIdOptions {
    pub organization_id: Uuid,
}

pub async fn get_one_organization_by_id(
    state: Arc<UroborosOauthState>,
    options: GetOneOrganizationByIdOptions,
) -> anyhow::Result<(organization_pg::Model, Vec<organization_member_pg::Model>)> {
    println!("get_one_organization_by_id.options {:?}", options);

    let o_organization = organization_pg::Entity::find_by_id(options.organization_id)
        .find_with_related(organization_member_pg::Entity)
        .all(&state.postgres)
        .await
        .map_err(|err| {
            println!("err {:?}", err);
            UroborosError {
                kind: UroborosErrorKind::CannotGet,
                message: "Cannot get organization".to_string(),
            }
        })?
        .first()
        .map(|organization| organization.clone());

    o_organization.ok_or(anyhow::anyhow!("Organization not found by id"))
}
