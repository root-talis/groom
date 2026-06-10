use axum::{
    Router,
    body::Body,
    http::{Method, Request, StatusCode},
};
use http_body_util::BodyExt;
use tower::ServiceExt;

use crate::{Bootstrap, make_router};

fn test_router() -> Router {
    make_router(Bootstrap::default(), false, None).expect("router should build")
}

struct HttpResponse {
    status: StatusCode,
    body: String,
}

async fn send(router: &Router, method: Method, url: &str, json_body: Option<&str>) -> HttpResponse {
    let mut builder = Request::builder().method(method).uri(url);
    let request = if let Some(body) = json_body {
        builder = builder.header("content-type", "application/json");
        builder.body(Body::from(body.to_owned())).unwrap()
    } else {
        builder.body(Body::empty()).unwrap()
    };

    let response = router.clone().oneshot(request).await.unwrap();
    let status = response.status();
    let body = response.into_body().collect().await.unwrap().to_bytes();

    HttpResponse {
        status,
        body: String::from_utf8(body.to_vec()).unwrap(),
    }
}

#[tokio::test]
async fn full_stack_handles_task_lifecycle() {
    // setup:
    let router = test_router();

    // --

    // when: create task
    let created = send(
        &router,
        Method::POST,
        "/tasks",
        Some(r#"{"title":"Buy milk"}"#),
    )
    .await;

    // then:
    assert_eq!(created.status, StatusCode::OK);
    let created_task: serde_json::Value = serde_json::from_str(&created.body).unwrap();
    assert_eq!(created_task["title"], "Buy milk");
    assert_eq!(created_task["status"], "Pending");
    let id = created_task["id"].as_u64().unwrap();

    // --

    // when: list tasks
    let listed = send(&router, Method::GET, "/tasks", None).await;

    // then:
    assert_eq!(listed.status, StatusCode::OK);
    let tasks: Vec<serde_json::Value> = serde_json::from_str(&listed.body).unwrap();
    assert_eq!(tasks.len(), 1);
    assert_eq!(tasks[0]["id"], id);
    assert_eq!(tasks[0]["title"], "Buy milk");
    assert_eq!(tasks[0]["status"], "Pending");

    // --

    // when: fetch task by id
    let fetched = send(&router, Method::GET, &format!("/tasks/{id}"), None).await;

    // then:
    assert_eq!(fetched.status, StatusCode::OK);
    let fetched_task: serde_json::Value = serde_json::from_str(&fetched.body).unwrap();
    assert_eq!(fetched_task["id"], id);
    assert_eq!(fetched_task["title"], "Buy milk");
    assert_eq!(fetched_task["status"], "Pending");

    // --

    // when: rename task
    let renamed = send(
        &router,
        Method::PUT,
        &format!("/tasks/{id}/name"),
        Some(r#"{"title":"Buy oat milk"}"#),
    )
    .await;

    // then:
    assert_eq!(renamed.status, StatusCode::OK);
    let renamed_task: serde_json::Value = serde_json::from_str(&renamed.body).unwrap();
    assert_eq!(renamed_task["id"], id);
    assert_eq!(renamed_task["title"], "Buy oat milk");
    assert_eq!(renamed_task["status"], "Pending");

    // --

    // when: mark task done
    let done = send(
        &router,
        Method::PUT,
        &format!("/tasks/{id}/status/done"),
        None,
    )
    .await;

    // then:
    assert_eq!(done.status, StatusCode::OK);
    let done_task: serde_json::Value = serde_json::from_str(&done.body).unwrap();
    assert_eq!(done_task["id"], id);
    assert_eq!(done_task["title"], "Buy oat milk");
    assert_eq!(done_task["status"], "Done");
}

#[tokio::test]
async fn get_missing_task_returns_not_found() {
    // setup:
    let router = test_router();

    // when:
    let response = send(&router, Method::GET, "/tasks/999", None).await;

    // then:
    assert_eq!(response.status, StatusCode::NOT_FOUND);
}
