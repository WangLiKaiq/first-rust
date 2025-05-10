use actix_web::dev::Server;
use actix_web::{App, HttpServer, web};
use lib::http::tracing::TraceMiddleware;
use lib::log::init_subscriber;
use std::net::TcpListener;

use crate::configuration::Settings;
use crate::test::test_endpoint::test;
pub struct Application {
    server: Server,
    port: u16,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, anyhow::Error> {
        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr()?.port();
        let server = run(listener).await?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

/**
 * We use this public function to expose the interface for test and user side.
 **/
async fn run(tcp_listener: TcpListener) -> Result<Server, anyhow::Error> {
    init_subscriber();
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TraceMiddleware)
            .route("/test/dummy", web::get().to(test))
    })
    .listen(tcp_listener)?
    .run();

    Ok(server)
}
