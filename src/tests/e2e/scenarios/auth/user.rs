use fake::Fake as _;

use crate::{
    apps::server::dtos::request::{auth::SetUserPasswordRequestBody, user::AddUserRequestBody},
    tests::e2e::core::test_client::TestClient,
    usecases::auth::sign_in::SignInByLoginOptions,
};

#[tokio::test]
async fn it_should_set_user_password_and_login() -> anyhow::Result<()> {
    let mut client = TestClient::default().await;
    client.login_as_default_admin().await?;

    let user_to_add = AddUserRequestBody::default();
    let user_id = client.add_user(&user_to_add).await?;

    let sign_in_options = SignInByLoginOptions {
        login: user_to_add.login.clone(),
        password: fake::faker::name::pt_br::NameWithTitle().fake::<String>(),
    };
    assert!(client
        .login_by_login_and_password(&sign_in_options)
        .await
        .is_err());

    client
        .set_user_password(&SetUserPasswordRequestBody {
            user_id: user_id.clone(),
            password: sign_in_options.password.clone(),
        })
        .await?;

    assert!(client
        .login_by_login_and_password(&sign_in_options)
        .await
        .is_ok());

    Ok(())
}
