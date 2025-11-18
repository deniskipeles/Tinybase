use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
};
use tower::ServiceExt;

mod common;
use common::setup_test_app;

#[tokio::test]
async fn test_create_collection() {
    let app = setup_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/collections")
                .header("content-type", "application/json")
                .body(Body::from(r#"{ "name": "Users" }"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
}

#[tokio::test]
async fn test_create_record() {
    let app = setup_test_app().await;

    // First, create a collection to get a valid ID
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/collections")
                .header("content-type", "application/json")
                .body(Body::from(r#"{ "name": "Posts" }"#))
                .unwrap(),
        )
        .await
        .unwrap();
    let body = to_bytes(response.into_body(), 1_048_576).await.unwrap();
    let collection: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let collection_id = collection["id"].as_i64().unwrap();

    // Now, create a record in that collection
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/v1/collections/{}/records", collection_id))
                .header("content-type", "application/json")
                .body(Body::from(r#"{ "data": { "title": "Hello!" } }"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
}
