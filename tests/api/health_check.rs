use crate::helpers::{spawn_app, TestApp};

#[actix_rt::test]
async fn health_check_works() {
    // Arrange
    let TestApp { address, .. } = spawn_app().await;

    // use reqwest to perform HTTP queries to the app
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
