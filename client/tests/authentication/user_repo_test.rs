use crate::context::app::AppTestContext;
use client::user::{
    authentication::{HashedPassword, PasswordSalt, RawPassword},
    *,
};
use lib::rand::rand_string;
use redis::Commands;
use secrecy::{ExposeSecret, SecretString};
use test_context::test_context;
use uuid::Uuid;

fn rand_save_user() -> SaveUser {
    SaveUser {
        id: Some(Uuid::new_v4()),
        username: Some(rand_string(16)),
        password: Some(
            HashedPassword::hash(
                &RawPassword(SecretString::from(rand_string(10))),
                PasswordSalt::rand(),
            )
            .unwrap(),
        ),
        email: Some(SecretString::from(rand_string(10))),
    }
}

/// How to fix: there is no reactor running, must be called from the context of a Tokio 1.x runtime
/// The root issue was the order in which Rust applies attribute macros.
/// They’re expanded from the bottom up (nearest the function first).
/// You need #[tokio::test] to run first—so it generates the async test with its Tokio runtime—and then
/// #[test_context(AppTestContext)] wraps that generated function to inject your setup/teardown.
/// ✅
///   #[test_context(AppTestContext)]
///   #[tokio::test]
///❌
///   #[tokio::test]
///   #[test_context(AppTestContext)]
#[test_context(AppTestContext)]
#[tokio::test]
/// test_context already have the async test mechanism, so #[tokio::test] is not required.
async fn should_insert_user_correctly(context: &mut AppTestContext) {
    let user = rand_save_user();
    let conn = context.state.db.clone();

    save_user(&conn, user.clone()).await.unwrap();

    let creds = get_stored_credentials(&user.username.unwrap(), &conn)
        .await
        .unwrap();
    assert!(creds.is_some());

    let (fetched_id, fetched_hash) = creds.unwrap();
    assert_eq!(fetched_id, user.id.unwrap());
    assert_eq!(
        fetched_hash.0.expose_secret(),
        user.password.unwrap().0.expose_secret()
    );
}

#[test_context(AppTestContext)]
#[tokio::test]
async fn should_update_the_user_correctly(context: &mut AppTestContext) {
    let user = rand_save_user();
    let conn = context.state.db.clone();
    save_user(&conn, user.clone()).await.unwrap();
    let updated_user = SaveUser {
        username: Some(rand_string(10)),
        ..user
    };

    save_user(&conn, updated_user.clone()).await.unwrap();
    let creds = get_stored_credentials(&updated_user.username.unwrap(), &conn)
        .await
        .unwrap();

    assert!(creds.is_some());
    let (fetched_id, fetched_hash) = creds.unwrap();
    assert_eq!(fetched_id, updated_user.id.unwrap());
    assert_eq!(
        fetched_hash.0.expose_secret(),
        updated_user.password.unwrap().0.expose_secret()
    );
}

#[test_context(AppTestContext)]
#[tokio::test]
async fn test(context: &mut AppTestContext) {
    let redis_client = context.state.redis_client.clone();
    let mut conn = redis_client.get_connection().unwrap();
    let token = Uuid::new_v4().to_string();

    // Save token with a TTL (e.g., 1 hour)
    conn.set_ex::<&str, &str, ()>("key", "value", 3600);
}
