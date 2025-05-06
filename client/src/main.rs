use lib::env::load_system_properties;
use lib::log::init_subscriber;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    load_system_properties();
    init_subscriber();
    Ok(())
}
