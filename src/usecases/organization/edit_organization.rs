use anyhow::{Context, Ok};
use sea_orm::{prelude::*, ActiveValue};
use std::sync::Arc;
use utoipa::ToSchema;

use crate::{
    adapters::postgres::entities::{organization_member_pg, organization_pg},
    apps::server::state::UroborosOauthState,
    domain::organization::member_role::OrganizationMemberRole,
    usecases::uroboros_user::get_user::{get_actor_by_id, GetActorByIdOptions},
};

#[derive(Debug, Default)]
pub struct AddOrganizationByUserOptions {
    pub actor_id: Uuid,
    pub name: String,
    pub description: Option<String>,
}

pub async fn add_organization_by_user(
    state: Arc<UroborosOauthState>,
    options: AddOrganizationByUserOptions,
) -> anyhow::Result<organization_pg::Model> {
    let actor = get_actor_by_id(
        state.clone(),
        GetActorByIdOptions {
            actor_id: options.actor_id,
        },
    )
    .await?;

    let orgainzation = add_organization(
        state.clone(),
        AddOrganizationOptions {
            name: options.name,
            description: options.description,
        },
    )
    .await?;

    add_organization_member(
        state.clone(),
        AddOrganizationMemberOptions {
            user_id: actor.id,
            organization_id: orgainzation.id,
            role: OrganizationMemberRole::Owner,
        },
    )
    .await?;

    Ok(orgainzation)
}

#[derive(Debug, Default, ToSchema)]
pub struct AddOrganizationOptions {
    pub name: String,
    pub description: Option<String>,
}

pub async fn add_organization(
    state: Arc<UroborosOauthState>,
    options: AddOrganizationOptions,
) -> anyhow::Result<organization_pg::Model> {
    let organization_to_add = organization_pg::ActiveModel {
        name: ActiveValue::Set(options.name),
        description: ActiveValue::Set(options.description),
        ..Default::default()
    };

    organization_to_add
        .insert(&state.postgres)
        .await
        .context("Cannot add organization")
}

#[derive(Debug, Default)]
pub struct AddOrganizationMemberByActorOptions {
    pub actor_id: Uuid,
    pub user_id: Uuid,
    pub organization_id: Uuid,
    pub role: OrganizationMemberRole,
}

pub async fn add_organization_member_by_actor(
    state: Arc<UroborosOauthState>,
    options: AddOrganizationMemberByActorOptions,
) -> anyhow::Result<organization_member_pg::Model> {
    let actor = get_actor_by_id(
        state.clone(),
        GetActorByIdOptions {
            actor_id: options.actor_id,
        },
    )
    .await?;

    println!("actor {:?}", actor);

    anyhow::bail!("Damn")
}

#[derive(Debug, Default)]
pub struct AddOrganizationMemberOptions {
    pub user_id: Uuid,
    pub organization_id: Uuid,
    pub role: OrganizationMemberRole,
}

pub async fn add_organization_member(
    state: Arc<UroborosOauthState>,
    options: AddOrganizationMemberOptions,
) -> anyhow::Result<organization_member_pg::Model> {
    let organization_member_to_add = organization_member_pg::ActiveModel {
        user_id: ActiveValue::Set(options.user_id),
        organization_id: ActiveValue::Set(options.organization_id),
        role: ActiveValue::Set(options.role),
        ..Default::default()
    };

    organization_member_to_add
        .insert(&state.postgres)
        .await
        .context("Cannot add organization member")
}
