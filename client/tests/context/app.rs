use client::{
    configure::app::AppConfig,
    server::{AppServer, AppState},
};
use lib::config::{env::get_profiles, read_config};
use test_context::AsyncTestContext;
use wiremock::MockServer;

pub struct AppTestContext {
    pub state: AppState,
    pub mock_server: MockServer,
}

impl AsyncTestContext for AppTestContext {
    async fn setup() -> Self {
        let config: AppConfig = read_config(get_profiles()).unwrap();
        let server = AppServer::new(config).await.unwrap();
        let state = server.state.clone();
        server.start().await.unwrap();
        let mock_server = MockServer::start().await;
        Self { state, mock_server }
    }

    async fn teardown(self) -> () {
        tracing::info!("Teardown done successfully.")
    }
}
