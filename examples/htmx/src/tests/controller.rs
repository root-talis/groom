use crate::{tests::test_utils::{Req, ReqBody}, make_router};

const DEFAULT_MESSAGE: &str = "Welcome to your message of the day.";

fn test_router() -> axum::Router {
    make_router().expect("router should build")
}

#[tokio::test]
async fn index_returns_default_message_and_view_count() {
    Req::get("/")
        .call(&test_router())
        .await
        .assert_status(200)
        .assert_content_type("text/html; charset=utf-8")
        .assert_body_contains(DEFAULT_MESSAGE)
        .assert_body_contains("Shown 1 time");
}

#[tokio::test]
async fn index_increments_view_count_on_each_visit() {
    let router = test_router();

    Req::get("/").call(&router).await.assert_status(200);

    Req::get("/")
        .call(&router)
        .await
        .assert_status(200)
        .assert_body_contains("Shown 2 times");
}

#[tokio::test]
async fn update_message_replaces_message_and_resets_view_count() {
    let router = test_router();

    Req::put("/message")
        .with_body(ReqBody::json(r#"{"message":"A fresh message for today."}"#))
        .call(&router)
        .await
        .assert_status(200)
        .assert_content_type("text/html; charset=utf-8")
        .assert_body_contains("A fresh message for today.")
        .assert_body_contains("Shown 0 times");

    Req::get("/")
        .call(&router)
        .await
        .assert_status(200)
        .assert_body_contains("A fresh message for today.")
        .assert_body_contains("Shown 1 time");
}

#[tokio::test]
async fn update_message_trims_whitespace() {
    Req::put("/message")
        .with_body(ReqBody::json(r#"{"message":"  padded message text  "}"#))
        .call(&test_router())
        .await
        .assert_status(200)
        .assert_body_contains("padded message text");
}

#[tokio::test]
async fn update_message_trims_unicode_whitespace() {
    let message = "\u{2003}unicode padded message\u{00a0}";

    Req::put("/message")
        .with_body(ReqBody::json(format!(r#"{{"message":"{message}"}}"#)))
        .call(&test_router())
        .await
        .assert_status(200)
        .assert_body_contains("unicode padded message");
}

#[tokio::test]
async fn update_message_rejects_short_unicode_message() {
    let message = "😀😀😀";

    Req::put("/message")
        .with_body(ReqBody::json(format!(r#"{{"message":"{message}"}}"#)))
        .call(&test_router())
        .await
        .assert_status(400)
        .assert_body_contains("New message is too short.");
}

#[tokio::test]
async fn update_message_accepts_minimum_valid_unicode_length() {
    let message = "😀😀😀😀";

    Req::put("/message")
        .with_body(ReqBody::json(format!(r#"{{"message":"{message}"}}"#)))
        .call(&test_router())
        .await
        .assert_status(200)
        .assert_body_contains(message);
}

#[tokio::test]
async fn update_message_rejects_long_unicode_message() {
    let message = "😀".repeat(512);

    Req::put("/message")
        .with_body(ReqBody::json(format!(r#"{{"message":"{message}"}}"#)))
        .call(&test_router())
        .await
        .assert_status(400)
        .assert_body_contains("New message is too long.");
}

#[tokio::test]
async fn update_message_accepts_maximum_valid_unicode_length() {
    let message = "😀".repeat(511);

    Req::put("/message")
        .with_body(ReqBody::json(format!(r#"{{"message":"{message}"}}"#)))
        .call(&test_router())
        .await
        .assert_status(200)
        .assert_body_contains(&message);
}

#[tokio::test]
async fn update_message_rejects_leading_slash() {
    Req::put("/message")
        .with_body(ReqBody::json(r#"{"message":"/not allowed"}"#))
        .call(&test_router())
        .await
        .assert_status(400)
        .assert_content_type("text/html; charset=utf-8")
        .assert_body_contains("New message starts with a slash symbol.");
}

#[tokio::test]
async fn update_message_rejects_short_message() {
    Req::put("/message")
        .with_body(ReqBody::json(r#"{"message":"hey"}"#))
        .call(&test_router())
        .await
        .assert_status(400)
        .assert_body_contains("New message is too short.");
}

#[tokio::test]
async fn update_message_accepts_minimum_valid_length() {
    Req::put("/message")
        .with_body(ReqBody::json(r#"{"message":"four"}"#))
        .call(&test_router())
        .await
        .assert_status(200)
        .assert_body_contains("four");
}

#[tokio::test]
async fn update_message_rejects_long_message() {
    let message = "x".repeat(512);

    Req::put("/message")
        .with_body(ReqBody::json(format!(r#"{{"message":"{message}"}}"#)))
        .call(&test_router())
        .await
        .assert_status(400)
        .assert_body_contains("New message is too long.");
}

#[tokio::test]
async fn update_message_accepts_maximum_valid_length() {
    let message = "x".repeat(511);

    Req::put("/message")
        .with_body(ReqBody::json(format!(r#"{{"message":"{message}"}}"#)))
        .call(&test_router())
        .await
        .assert_status(200)
        .assert_body_contains(&message);
}
