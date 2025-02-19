use crate::{
    apps::server::dtos::request::user::AddUserRequestBody,
    tests::e2e::core::test_client::TestClient,
};

#[tokio::test]
async fn it_should_add_usroboros_user_by_default_admin() -> anyhow::Result<()> {
    let mut client = TestClient::default().await;
    client.login_as_default_admin().await?;

    let user_to_add = AddUserRequestBody::default();
    let user_id = client.add_user(&user_to_add).await?;

    let got_user = client.get_user_by_id(&user_id).await?;

    assert_eq!(got_user.id.to_string(), user_id);
    assert_eq!(got_user.first_name, user_to_add.first_name);
    assert_eq!(got_user.last_name, user_to_add.last_name);
    assert_eq!(got_user.patronymick, user_to_add.patronymick);

    Ok(())
}
