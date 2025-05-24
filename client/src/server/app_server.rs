use actix_web::middleware::from_fn;
use actix_web::{App, HttpServer, web};
use lib::http::tracing::TraceMiddleware;
use lib::log::init_subscriber;
use state::AppState;

use crate::configure::app::AppConfig;
use crate::user::authentication::reject_anonymous_users;

use super::router::test::test;
use super::router::user::user_router;
use super::state;
pub struct AppServer {
    pub state: AppState,
    tcp: std::net::TcpListener,
}

impl AppServer {
    pub async fn new(configuration: AppConfig) -> Result<Self, anyhow::Error> {
        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        tracing::info!("Server will start on {}", address);
        let listener = std::net::TcpListener::bind(address)?;
        let state = AppState::new(configuration).await?;
        Ok(Self {
            tcp: listener,
            state,
        })
    }

    /**
     * We use this public function to expose the interface for test and user side.
     **/
    pub async fn start(self) -> Result<(), anyhow::Error> {
        init_subscriber();

        let server = HttpServer::new(move || {
            App::new()
                .wrap(TraceMiddleware)
                .route(
                    "/test/dummy",
                    web::get().to(test).wrap(from_fn(reject_anonymous_users)),
                )
                .service(user_router())
                .app_data(web::Data::new(self.state.clone()))
        })
        .listen(self.tcp)?
        .run();
        server.await.unwrap();
        Ok(())
    }
}
