use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sea_orm::prelude::Uuid;

use crate::{
    apps::server::{
        dtos::{
            request::organization::{AddOrganizationRouteBody, GetOrganizationPageRouteQuery},
            response::map::map_entity_result_to_response_tuple,
        },
        state::UroborosOauthState,
    },
    domain::jwt::JwtClaims,
    usecases::organization::{
        edit_organization::{add_organization_by_user, AddOrganizationByUserOptions},
        get_organization::{
            get_one_organization_by_id, get_organization_members_by_id,
            get_organization_page_by_user, GetOneOrganizationByIdOptions,
            GetOrganizationPageByUserOptions,
        },
    },
};

#[utoipa::path(
    get,
    path = "/organization",
    params(GetOrganizationPageRouteQuery),
    responses(
        (status = 200, description = "Get organizations page")
    ),
    security(("token" = []))
)]
pub async fn get_organization_page_route(
    jwt: JwtClaims,
    State(state): State<Arc<UroborosOauthState>>,
    Query(query): Query<GetOrganizationPageRouteQuery>,
) -> impl IntoResponse {
    let result = get_organization_page_by_user(
        state,
        GetOrganizationPageByUserOptions {
            actor_id: jwt.uid.parse().unwrap_or_default(),
            page: query.page.unwrap_or(1_u64),
        },
    )
    .await;

    map_entity_result_to_response_tuple(StatusCode::OK, result)
}

#[utoipa::path(
    post,
    path = "/organization",
    request_body(content=AddOrganizationRouteBody),
    responses((status = 200, description = "Add organization")),
    security(("token" = []))
)]
pub async fn add_organization_route(
    jwt: JwtClaims,
    State(state): State<Arc<UroborosOauthState>>,
    Json(body): Json<AddOrganizationRouteBody>,
) -> impl IntoResponse {
    let result = add_organization_by_user(
        state.clone(),
        AddOrganizationByUserOptions {
            actor_id: jwt.uid.parse().unwrap_or_default(),
            name: body.name,
            description: body.description,
        },
    )
    .await;

    map_entity_result_to_response_tuple(StatusCode::OK, result)
}

#[utoipa::path(
    get,
    path = "/organization/{organization_id}",
    params(("organization_id" = String, Path)),
    responses(
        (status = 200, description = "Get one organization by id")
    ),
    security(("token" = []))
)]
pub async fn get_one_organization_by_id_route(
    // jwt: JwtClaims,
    State(state): State<Arc<UroborosOauthState>>,
    Path(organization_id): Path<Uuid>,
) -> impl IntoResponse {
    let r_organization = get_one_organization_by_id(
        state.clone(),
        GetOneOrganizationByIdOptions { organization_id },
    )
    .await;

    map_entity_result_to_response_tuple(
        StatusCode::CREATED,
        r_organization.map(|(organization, _)| organization),
    )
}

#[utoipa::path(
    get,
    path = "/organization/{organization_id}/member",
    params(("organization_id" = String, Path)),
    responses(
        (status = 200, description = "Add organization member")
    ),
    security(("token" = []))
)]
pub async fn get_one_organization_members_list_route(
    // jwt: JwtClaims,
    State(state): State<Arc<UroborosOauthState>>,
    Path(organization_id): Path<Uuid>,
) -> impl IntoResponse {
    // TODO: Check rights
    let r_members = get_organization_members_by_id(
        state.clone(),
        GetOneOrganizationByIdOptions { organization_id },
    )
    .await;
    map_entity_result_to_response_tuple(StatusCode::OK, r_members)
}

#[utoipa::path(
    post,
    path = "/organization/{organization_id}/member",
    params(("organization_id" = String, Path)),
    responses(
        (status = 200, description = "Add organization member")
    ),
    security(("token" = []))
)]
pub async fn add_one_organization_member_route(// jwt: JwtClaims,
    // State(state): State<Arc<UroborosOauthState>>,
    // Path(organization_id): Path<Uuid>,
) -> impl IntoResponse {
    ""
}
