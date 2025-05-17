use client::{constant::CONFIG, server::AppServer};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let server = AppServer::new(CONFIG.clone()).await?;

    server.start().await?;
    Ok(())
}
