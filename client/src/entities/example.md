## Examples

```rust
mod entities;
use chrono::Utc;
use dotenvy::dotenv;
use entities::{prelude::*, *};
use sea_orm::*;
use uuid::Uuid;
async fn demo() -> Result<(), anyhow::Error> {
    dotenv().ok();
    let url = std::env::var("DATABASE_URL").expect("can'T get the database URL");
    let db = Database::connect(url).await.expect("");
    let happy_bakery = users::ActiveModel {
        id: Set(Uuid::new_v4().as_bytes().to_vec()),
        username: Set("ABCD".to_owned()),
        password_hash: Set("...hashed...".to_owned()),
        email: Set("...hashed...".to_owned()),
        created_at: Set(Some(Utc::now())),
    };
    Users::insert(happy_bakery).exec(&db).await.expect("msg");

    let res = users::Entity::find().one(&db).await.expect("msg");
    dbg!(res);
    Ok(())
}

```