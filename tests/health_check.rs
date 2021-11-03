use std::net::TcpListener;

#[actix_rt::test]
async fn health_check_works() {
    // Arrange
    let address = spawn_app();

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

// Launch our application in the background
fn spawn_app() -> String {
    // use port `0` for a random available port provided by the OS
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    // Retrieve the port assigned by the OS
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to bind address");
    // spawn a background thread that runs our server
    let _ = tokio::spawn(server);
    // Return the application address to the caller
    format!("http://127.0.0.1:{}", port)
}
