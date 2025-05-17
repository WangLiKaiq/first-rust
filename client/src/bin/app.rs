use client::{configure::app::AppConfig, startup::Application};
use lib::config::{env::get_profiles, read_config};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let profiles = get_profiles();
    let configuration: AppConfig = read_config(profiles)?;
    let application = Application::build(configuration).await?;

    application.run_until_stopped().await?;
    Ok(())
}
