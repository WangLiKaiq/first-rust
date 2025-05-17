pub mod state;

use actix_web::dev::Server;
use actix_web::{App, HttpServer, web};
use lib::http::tracing::TraceMiddleware;
use lib::log::init_subscriber;
use state::AppState;
use std::net::TcpListener;

use crate::configure::app::AppConfig;
use crate::handler::user;
use crate::router::user::user_router;
use crate::test::test_endpoint::test;
pub struct AppServer {
    pub state: AppState,
    server: Server,
    port: u16,
}

impl AppServer {
    pub async fn new(configuration: AppConfig) -> Result<Self, anyhow::Error> {
        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr()?.port();
        let state = AppState::new(configuration).await?;
        let server = Self::run(state.clone(), listener).await?;
        Ok(Self {
            port,
            server,
            state,
        })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }

    /**
     * We use this public function to expose the interface for test and user side.
     **/
    async fn run(state: AppState, tcp_listener: TcpListener) -> Result<Server, anyhow::Error> {
        init_subscriber();
        let server = HttpServer::new(move || {
            App::new()
                .wrap(TraceMiddleware)
                .route("/test/dummy", web::get().to(test))
                .service(user_router())
                .app_data(web::Data::new(state.clone()))
        })
        .listen(tcp_listener)?
        .run();

        Ok(server)
    }
}
