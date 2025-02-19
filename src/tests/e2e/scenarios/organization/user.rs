use anyhow::Context;
use fake::Fake;

use crate::{
    apps::server::dtos::request::organization::AddOrganizationRouteBody,
    domain::uroboros_user::uroboros_role::UroborosUserRole,
    tests::e2e::{
        core::{fake::get_timestamp_postfix, test_client::TestClient},
        seed::user::get_test_client_token_by_role,
    },
};

#[tokio::test]
async fn it_should_add_organization() -> anyhow::Result<()> {
    let mut client = TestClient::default().await;
    client.set_token(
        get_test_client_token_by_role(client.get_state(), UroborosUserRole::User).await?,
    );

    let organization_draft = AddOrganizationRouteBody {
        name: fake::faker::company::fr_fr::CompanyName().fake::<String>()
            + get_timestamp_postfix().as_str(),
        description: None,
    };

    let organization = client.add_organization(&organization_draft).await?;

    assert_eq!(organization.name, organization_draft.name);
    assert_eq!(organization.description, organization_draft.description);

    let actor = client.get_current_user().await?;
    let members = client
        .get_organization_members_list(&organization.id.to_string())
        .await?
        .context("Cannto get members list")?;

    assert_eq!(members.len(), 1);
    assert!(members.iter().any(|member| member.user_id == actor.id));

    Ok(())
}
