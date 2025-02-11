use std::sync::Arc;

use axum::Router;
use dotenv::dotenv;

use crate::apps::server::router::create_router;
use crate::apps::server::state::UroborosOauthState;
use crate::usecases::init::init;
use tokio::task;

pub async fn create_app_and_state(with_init: bool) -> (Router<()>, Arc<UroborosOauthState>) {
    dotenv().ok();

    let state: Arc<UroborosOauthState> = Arc::new(UroborosOauthState::from_env().await);

    if with_init {
        let init_state = state.clone();
        task::spawn(async {
            let _ = init(init_state).await;
        });
    }

    (create_router(state.clone()), state)
}
