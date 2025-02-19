use crate::{
    apps::server::dtos::response::auth::TokenResponse,
    tests::e2e::core::test_server::get_test_server_and_state,
    usecases::auth::sign_in::SignInByLoginOptions,
};

#[tokio::test]
async fn it_should_login_as_default_admin() {
    let (test_server, state) = get_test_server_and_state().await;

    assert!(
        state.clone().default_admin_options.is_some(),
        "there is no default_admin options"
    );

    let default_admin_options = state.default_admin_options.as_ref().unwrap().clone();

    assert_ne!(default_admin_options.login.len(), 0);
    assert_ne!(default_admin_options.password.len(), 0);

    let response = test_server
        .post("/auth/sign-in")
        .json(&SignInByLoginOptions {
            login: default_admin_options.login.clone(),
            password: default_admin_options.password.clone(),
        })
        .await;

    response.assert_status_ok();

    let response_body = response.json::<TokenResponse>();

    assert!(response_body.token.is_some());
    assertables::assert_ge!(response_body.token.unwrap_or_default().len(), 32);
}
