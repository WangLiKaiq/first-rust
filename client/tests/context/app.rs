use std::net::{SocketAddr, TcpListener};

use client::{
    configure::app::AppConfig,
    server::{AppServer, AppState},
};
use lib::config::{env::get_profiles, read_config};
use test_context::AsyncTestContext;
use wiremock::MockServer;

pub struct AppTestContext {
    pub config: AppConfig,
    pub state: AppState,
    pub mock_server: MockServer,
}

impl AsyncTestContext for AppTestContext {
    async fn setup() -> Self {
        let config: AppConfig = {
            let mut c: AppConfig = read_config(get_profiles()).unwrap();
            let port = pick_free_port();
            c.application.port = port;
            c
        };

        let server = AppServer::new(config.clone()).await.unwrap();
        let state = server.state.clone();
        tokio::spawn(async move {
            let _ = server.start().await;
        });

        let mock_server = MockServer::start().await;

        AppTestContext {
            config,
            state,
            mock_server,
        }
    }

    async fn teardown(self) {
        tracing::info!("Teardown done successfully.");
    }
}

fn pick_free_port() -> u16 {
    let listener =
        std::net::TcpListener::bind("127.0.0.1:0").expect("failed to bind to ephemeral port");
    let port = listener.local_addr().unwrap().port();
    drop(listener);
    port
}
