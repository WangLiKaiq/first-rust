use sea_orm::{Database, DatabaseConnection};

pub async fn get_conn_from_config() -> Result<DatabaseConnection, anyhow::Error> {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");
    let db = Database::connect(&db_url).await?;
    Ok(db)
}
