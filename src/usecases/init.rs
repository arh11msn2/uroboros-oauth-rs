use std::sync::Arc;

use crate::apps::server::state::UroborosOauthState;

use super::uroboros_user::ensure_default_admin::ensure_default_admin;

pub async fn init(state: Arc<UroborosOauthState>) -> anyhow::Result<()> {
    ensure_default_admin(state.clone()).await?;
    Ok(())
}
