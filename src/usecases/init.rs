use std::sync::Arc;

use crate::{apps::server::state::UroborosOauthState, domain::result::UroborosResult};

use super::uroboros_user::ensure_superadmin::ensure_superadmin;

pub async fn init(state: Arc<UroborosOauthState>) -> UroborosResult<()> {
    ensure_superadmin(state.clone()).await?;
    Ok(())
}
