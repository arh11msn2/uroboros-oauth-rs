use anyhow::Context;
use sea_orm::{prelude::*, QuerySelect};
use std::sync::Arc;

use crate::{
    adapters::postgres::entities::{organization_member_pg, organization_pg},
    apps::server::state::UroborosOauthState,
    domain::result::{UroborosError, UroborosErrorKind},
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
) -> anyhow::Result<Vec<organization_pg::Model>> {
    let _actor = get_actor_by_id(
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
        .context("Cannot get organizations page")
}

#[derive(Debug, Default)]
pub struct GetOneOrganizationByIdOptions {
    pub organization_id: Uuid,
}

pub async fn get_one_organization_by_id(
    state: Arc<UroborosOauthState>,
    options: GetOneOrganizationByIdOptions,
) -> anyhow::Result<(organization_pg::Model, Vec<organization_member_pg::Model>)> {
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
        .cloned();

    o_organization.ok_or(anyhow::anyhow!("Organization not found by id"))
}

pub async fn get_organization_members_by_id(
    state: Arc<UroborosOauthState>,
    options: GetOneOrganizationByIdOptions,
) -> anyhow::Result<Vec<organization_member_pg::Model>> {
    organization_member_pg::Entity::find()
        .filter(organization_member_pg::Column::OrganizationId.eq(options.organization_id))
        .all(&state.postgres)
        .await
        .context("Cannot get organization members")
}
