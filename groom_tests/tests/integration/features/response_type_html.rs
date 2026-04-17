use axum::Router;

use crate::{
    groom_macros::Controller,
    integration::{features::response_type_html::controller::SOME_TEXT, test_utils::Req}
};

#[Controller()]
mod controller {
    use axum::response::IntoResponse;
    use groom::{
        schema::GroomSchema,
        html_format,
        response::Response
    };
    use groom_macros::{DTO,Response};

    pub const SOME_TEXT: &str = "<strong>apes strong together</strong>";

    // ---

    #[DTO(response)]
    pub struct DataObject {
        pub status: &'static str,
        pub status_timestamp: u64,
    }

    impl Default for DataObject {
        fn default() -> Self {
            Self { 
                status: "ok",
                status_timestamp: 1726070400,
            }
        }
    }

    html_format!(DataObject, self {
        // important: in production make sure to escape special chars!
        format!("status: <b>{}</b> (since <b>{}</b>)", self.status, self.status_timestamp)
    });

    // ---

    #[Response(format(html))]
    pub enum HtmlStringResponse {
        #[Response()]
        Ok(String)
    }

    #[Route(method="get", path="/html_string")]
    pub async fn html_string() -> HtmlStringResponse {
        HtmlStringResponse::Ok(SOME_TEXT.into())
    }

    // ---

    #[Response(format(html))]
    pub enum HtmlStructResponse {
        #[Response()]
        Ok(DataObject)
    }

    #[Route(method="get", path="/html_struct")]
    pub async fn html_struct() -> HtmlStructResponse {
        HtmlStructResponse::Ok(DataObject::default())
    }
}

/// Tests that handler for delete request is set correctly
#[tokio::test]
pub async fn html_string() {
    let r = controller::merge_into_router(Router::new());

    Req::get("/html_string").call(&r).await
        .assert_body(SOME_TEXT)
        .assert_status(200)
        .assert_content_type("text/html; charset=utf-8")
    ;
}

/// Tests that handler for delete request is set correctly
#[tokio::test]
pub async fn html_struct() {
    let r = controller::merge_into_router(Router::new());

    Req::get("/html_struct").call(&r).await
        .assert_body("status: <b>ok</b> (since <b>1726070400</b>)")
        .assert_status(200)
        .assert_content_type("text/html; charset=utf-8")
    ;
}

