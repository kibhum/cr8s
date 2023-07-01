use reqwest::{blocking::Client, StatusCode};
use rocket::serde::json::{serde_json::json, Value};

#[test]
fn test_get_rustaceans() {
    let client = Client::new();
    let response = client
        .get("http://127.0.0.1:8000/rustaceans")
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK)
}
