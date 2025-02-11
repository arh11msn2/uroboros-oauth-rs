use crate::{
    apps::server::routes::auth::TokenResponse,
    tests::e2e::core::test_server::get_test_server_and_state,
    usecases::auth::sign_in::SignInByLoginOptions,
};

#[tokio::test]
async fn it_should_login_as_superadmin() {
    let (test_server, state) = get_test_server_and_state().await;

    assert!(
        state.clone().superadmin_options.is_some(),
        "there is no superadmin options"
    );

    let superadmin_options = state.superadmin_options.as_ref().unwrap().clone();

    assert_ne!(superadmin_options.login.len(), 0);
    assert_ne!(superadmin_options.password.len(), 0);

    let response = test_server
        .post("/sign-in")
        .json(&SignInByLoginOptions {
            login: superadmin_options.login.clone(),
            password: superadmin_options.password.clone(),
        })
        .await;

    response.assert_status_ok();

    let response_body = response.json::<TokenResponse>();

    assert!(response_body.token.is_some());
    assertables::assert_ge!(response_body.token.unwrap_or_default().len(), 32);
}
