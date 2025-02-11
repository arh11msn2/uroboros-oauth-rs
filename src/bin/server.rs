use uroboros::apps::server::app::create_app_and_state;

#[tokio::main]
async fn main() {
    let (app, state) = create_app_and_state(true).await;

    let listener = tokio::net::TcpListener::bind(state.clone().server_options.addr())
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
