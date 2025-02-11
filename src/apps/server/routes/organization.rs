use std::sync::Arc;

use axum::{
    body::{self, Body},
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sea_orm::prelude::Uuid;
use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa::{IntoParams, ToSchema};

use crate::{
    adapters::postgres::entities::organization_pg,
    apps::server::state::UroborosOauthState,
    domain::jwt::JwtClaims,
    usecases::organization::{
        edit_organization::{add_organization_by_user, AddOrganizationByUserOptions},
        get_organization::{
            get_one_organization_by_id, get_organization_page_by_user,
            GetOneOrganizationByIdOptions, GetOrganizationPageByUserOptions,
        },
    },
};

#[derive(Debug, Deserialize, ToSchema, IntoParams)]
pub struct GetOrganizationPageRouteQuery {
    pub page: Option<u64>,
}

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
    let organizations_result = get_organization_page_by_user(
        state,
        GetOrganizationPageByUserOptions {
            actor_id: jwt.uid.parse().unwrap_or_default(),
            page: query.page.unwrap_or(1_u64),
        },
    )
    .await;

    match organizations_result {
        Ok(organizations) => (StatusCode::OK, Json(organizations)),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(Vec::new())),
    }
}

#[derive(Debug, Deserialize, ToSchema, IntoParams)]
pub struct AddOrganizationRouteBody {
    pub name: String,
    pub description: Option<String>,
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

    match result {
        Ok(organization) => (StatusCode::OK, Json(Some(organization))),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(None)),
    }
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
    jwt: JwtClaims,
    State(state): State<Arc<UroborosOauthState>>,
    Path(organization_id): Path<Uuid>,
) -> impl IntoResponse {
    println!("hook");
    let r_organization = get_one_organization_by_id(
        state.clone(),
        GetOneOrganizationByIdOptions { organization_id },
    )
    .await;

    match r_organization {
        Ok(organization) => (StatusCode::INTERNAL_SERVER_ERROR, Json(organization)).into_response(),
        Err(err) => {
            println!("error: {:?}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "err": err.to_string() })),
            )
                .into_response()
        }
    }
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
pub async fn add_one_organization_member_route(
    jwt: JwtClaims,
    State(state): State<Arc<UroborosOauthState>>,
    Path(organization_id): Path<Uuid>,
) -> impl IntoResponse {
    "".into_response()
}
