use std::sync::Arc;

use anyhow::Context;
use axum::http::{HeaderName, HeaderValue, Method, StatusCode};
use axum_test::TestServer;

use crate::{
    adapters::postgres::entities::{organization_member_pg, organization_pg, user_pg},
    apps::server::{
        dtos::{
            request::{
                auth::SetUserPasswordRequestBody, organization::AddOrganizationRouteBody,
                user::AddUserRequestBody,
            },
            response::auth::TokenResponse,
        },
        state::UroborosOauthState,
    },
    ports::dtos::core::responses::{OkResponse, StringIdResponse},
    usecases::auth::sign_in::SignInByLoginOptions,
};

use super::test_server::get_test_server_and_state;

#[cfg(test)]
#[derive(Debug, Clone)]
pub struct TestClient {
    server: Arc<TestServer>,
    state: Arc<UroborosOauthState>,
    token: Option<String>,
}

#[cfg(test)]
impl TestClient {
    pub async fn default() -> Self {
        let (server, state) = get_test_server_and_state().await;

        Self {
            server: Arc::new(server),
            state,
            token: None,
        }
    }

    pub fn reset_token(&mut self) {
        self.token = None;
    }

    pub fn set_token(&mut self, token: String) {
        self.token = Some(token);
    }

    pub fn get_server(&self) -> Arc<TestServer> {
        self.server.clone()
    }

    pub fn get_state(&self) -> Arc<UroborosOauthState> {
        self.state.clone()
    }

    pub fn make_request(
        &self,
        method: Method,
        path: &str,
        use_token: bool,
    ) -> anyhow::Result<axum_test::TestRequest> {
        let mut request: axum_test::TestRequest = self.server.method(method, path);

        if use_token {
            request = request.add_header(
                HeaderName::from_bytes("Authorization".as_bytes())?,
                HeaderValue::from_str(&format!(
                    "Bearer {}",
                    self.token.clone().context("Unauthorized")?
                ))?,
            );
        }

        Ok(request)
    }

    pub async fn login_by_login_and_password(
        &mut self,
        body: &SignInByLoginOptions,
    ) -> anyhow::Result<()> {
        let response = self.server.post("/auth/sign-in").json(&body).await;

        let token = response
            .json::<TokenResponse>()
            .token
            .context("Jwt token not found")?;

        self.set_token(token);

        Ok(())
    }

    pub async fn login_as_default_admin(&mut self) -> anyhow::Result<()> {
        let default_admin_options = self.state.default_admin_options.as_ref().unwrap().clone();

        let body = SignInByLoginOptions {
            login: default_admin_options.login.clone(),
            password: default_admin_options.password.clone(),
        };

        self.login_by_login_and_password(&body).await
    }

    pub async fn add_user(&self, body: &AddUserRequestBody) -> anyhow::Result<String> {
        let response = self.server.post("/user").json(body).await;

        response.assert_status(StatusCode::CREATED);

        let user_id = response.json::<StringIdResponse>().id;

        Ok(user_id)
    }

    pub async fn get_current_user(&self) -> anyhow::Result<user_pg::Model> {
        let response = self.make_request(Method::GET, "/user/me", true)?.await;

        response.assert_status(StatusCode::OK);

        Ok(response.json::<user_pg::Model>())
    }

    pub async fn get_user_by_id(&self, user_id: &str) -> anyhow::Result<user_pg::Model> {
        let response = self
            .make_request(Method::GET, &format!("/user/{}", user_id), true)?
            .await;

        response.assert_status(StatusCode::OK);

        Ok(response.json::<user_pg::Model>())
    }

    pub async fn set_user_password(
        &self,
        body: &SetUserPasswordRequestBody,
    ) -> anyhow::Result<OkResponse> {
        let response = self
            .make_request(Method::POST, "/auth/password", false)?
            .json(body)
            .await;

        response.assert_status(StatusCode::OK);

        println!("{:?}", response);

        Ok(response.json::<OkResponse>())
    }

    pub async fn add_organization(
        &self,
        body: &AddOrganizationRouteBody,
    ) -> anyhow::Result<organization_pg::Model> {
        let response = self
            .make_request(Method::POST, "/organization", true)?
            .json(body)
            .await;

        response.assert_status(StatusCode::OK);

        response
            .json::<Option<organization_pg::Model>>()
            .context("Organization not found in response")
    }

    pub async fn get_organization_members_list(
        &self,
        organization_id: &str,
    ) -> anyhow::Result<Option<Vec<organization_member_pg::Model>>> {
        let response = self
            .make_request(
                Method::GET,
                &format!("/organization/{organization_id:}/member"),
                true,
            )?
            .await;

        response.assert_status(StatusCode::OK);

        response
            .json::<Option<Option<Vec<organization_member_pg::Model>>>>()
            .context("Organization not found in response")
    }
}
