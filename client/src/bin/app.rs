use client::{constant::CONFIG, server::AppServer};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let server = AppServer::new(CONFIG.clone()).await?;

    server.run_until_stopped().await?;
    Ok(())
}
