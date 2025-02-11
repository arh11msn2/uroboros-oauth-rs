use std::sync::Arc;

use axum_test::TestServer;

use crate::apps::server::state::UroborosOauthState;

pub async fn get_test_server_and_state() -> (TestServer, Arc<UroborosOauthState>) {
    use crate::apps::server::app::create_app_and_state;

    let (app, state) = create_app_and_state(false).await;

    (axum_test::TestServer::new(app).unwrap(), state)
}
