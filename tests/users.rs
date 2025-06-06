use crate::common::wait_between_tests;
use jikan_rs::JikanClient;
use jikan_rs::users::*;
use serial_test::serial;
mod common;

#[tokio::test]
#[serial]
async fn get_user_full() {
    let client = JikanClient::new();
    let result = client.get_user_full("InSaiyan__").await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_user() {
    let client = JikanClient::new();
    let result = client.get_user("InSaiyan__").await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_users() {
    let client = JikanClient::new();
    let result = client
        .get_users(GetUsersParams {
            page: None,
            limit: None,
            q: None,
            gender: Gender::None,
            location: None,
            min_age: None,
            max_age: None,
        })
        .await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_user_by_id() {
    let client = JikanClient::new();
    let result = client.get_user_by_id(15847568).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_user_stats() {
    let client = JikanClient::new();
    let result = client.get_user_stats("InSaiyan__").await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_user_friends() {
    let client = JikanClient::new();
    let result = client.get_user_friends("Kisaragi_Toka", Some(1)).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_user_reviews() {
    let client = JikanClient::new();
    let result = client.get_user_reviews("eren", Some(1)).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_user_history() {
    let client = JikanClient::new();
    let result = client
        .get_user_history("Kisaragi_Toka", UserHistoryType::None)
        .await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_user_favorites() {
    let client = JikanClient::new();
    let result = client.get_user_favorites("Kisaragi_Toka").await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_user_about() {
    let client = JikanClient::new();
    let result = client.get_user_about("eren").await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_user_external() {
    let client = JikanClient::new();
    let result = client.get_user_external("Kisaragi_Toka").await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_user_updates() {
    let client = JikanClient::new();
    let result = client.get_user_updates("Kisaragi_Toka").await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_user_recommendations() {
    let client = JikanClient::new();
    let result = client
        .get_user_recommendations("Kisaragi_Toka", Some(1))
        .await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_user_clubs() {
    let client = JikanClient::new();
    let result = client.get_user_clubs("Kisaragi_Toka", Some(1)).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}
