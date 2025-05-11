use client::{config::Settings, startup::Application};
use lib::config::get_configuration;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let configuration: Settings = get_configuration().expect("Failed to load configuration");
    let application = Application::build(configuration).await?;

    application.run_until_stopped().await?;
    Ok(())
}
