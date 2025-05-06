use lib::log::init_subscriber;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    init_subscriber();
    Ok(())
}
