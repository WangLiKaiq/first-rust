use client::authentication::{
    HashedPassword, PasswordSalt, RawPassword, SaveUser, get_stored_credentials, save_user,
};
use lib::rand::rand_string;
use secrecy::{ExposeSecret, SecretString};
use test_context::test_context;
use uuid::Uuid;

use crate::context::app::AppTestContext;

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
#[tokio::test]
/// test_context already have the async test mechanism, so #[tokio::test] is not required.
async fn should_insert_user_correctly() {
    let context = AppTestContext::setup().await;
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

// #[test_context(AppTestContext)]
// async fn should_update_the_user_correctly(context: &mut AppTestContext) {
//     let user = rand_save_user();
//     let conn = context.state.db.clone();
//     save_user(&conn, user.clone()).await.unwrap();
//     let updated_user = SaveUser {
//         username: Some(rand_string(10)),
//         ..user
//     };

//     save_user(&conn, updated_user.clone()).await.unwrap();
//     let creds = get_stored_credentials(&updated_user.username.unwrap(), &conn)
//         .await
//         .unwrap();

//     assert!(creds.is_some());
//     let (fetched_id, fetched_hash) = creds.unwrap();
//     assert_eq!(fetched_id, updated_user.id.unwrap());
//     assert_eq!(
//         fetched_hash.0.expose_secret(),
//         updated_user.password.unwrap().0.expose_secret()
//     );
// }
