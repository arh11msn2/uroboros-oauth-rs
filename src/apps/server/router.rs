use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::{Modify, OpenApi};
use utoipa_swagger_ui::SwaggerUi;

use super::routes::auth::*;
use super::routes::index::*;
use super::routes::organization::*;
use super::routes::uroboros_user::*;
use crate::apps::server::state::UroborosOauthState;

#[derive(OpenApi)]
#[openapi(
    paths(
        get_index_route,
        get_organization_page_route, add_organization_route,
        get_one_organization_by_id_route,
        add_one_organization_member_route, get_one_organization_members_list_route,
        sign_in_route, set_user_password_route,
        add_uroborus_user_route, get_uroborus_users_list_route,
        get_current_user_route, get_one_user_by_id_route,
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "token",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        )
    }
}

pub fn create_router(state: Arc<UroborosOauthState>) -> Router<()> {
    Router::new()
        .route("/", get(get_index_route))
        .route("/auth/sign-in", post(sign_in_route))
        .route("/auth/password", post(set_user_password_route))
        .route(
            "/organization",
            get(get_organization_page_route).post(add_organization_route),
        )
        .route(
            "/organization/:organization_id",
            get(get_one_organization_by_id_route),
        )
        .route(
            "/organization/:organization_id/member",
            get(get_one_organization_members_list_route).post(add_one_organization_member_route),
        )
        .route(
            "/user",
            get(get_uroborus_users_list_route).post(add_uroborus_user_route),
        )
        .route("/user/me", get(get_current_user_route))
        .route("/user/:user_id", get(get_one_user_by_id_route))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", ApiDoc::openapi()))
        .with_state(state)
}
