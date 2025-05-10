use client::{configuration::get_configuration, startup::Application};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let configuration = get_configuration().expect("Failed to load configuration");
    let application = Application::build(configuration).await?;

    application.run_until_stopped().await?;
    Ok(())
}
