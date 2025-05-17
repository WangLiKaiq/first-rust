use client::{
    configure::app::AppConfig,
    server::{AppServer, AppState},
};
use lib::config::{env::get_profiles, read_config};
use test_context::AsyncTestContext;
use tokio::task::JoinHandle;
use wiremock::MockServer;

pub struct AppTestContext {
    pub tasks: Vec<JoinHandle<Result<(), std::io::Error>>>,
    pub state: AppState,
    pub mock_server: MockServer,
}

impl AsyncTestContext for AppTestContext {
    async fn setup() -> Self {
        let config: AppConfig = read_config(get_profiles()).unwrap();
        let server = AppServer::new(config).await.unwrap();
        let state = server.state.clone();
        let server_task = tokio::task::spawn(server.run_until_stopped());
        let tasks = vec![server_task];
        let mock_server = MockServer::start().await;
        Self {
            tasks,
            state,
            mock_server,
        }
    }

    async fn teardown(self) -> () {
        for j in self.tasks {
            j.abort();
        }
        tracing::info!("Teardown done successfully.")
    }
}
