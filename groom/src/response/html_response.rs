use axum::{body::Body, response::Html};

pub trait HtmlFormat {
    fn render(self) -> Html<Body>;
}

impl HtmlFormat for String {
    fn render(self) -> ::axum::response::Html<axum::body::Body> {
        ::axum::response::Html(self.into())
    }
}

impl HtmlFormat for &'static str {
    fn render(self) -> ::axum::response::Html<axum::body::Body> {
        ::axum::response::Html(self.into())
    }
}

/// Implements HtmlFormat interface for custom type.
///
/// Useful for templating.
///
/// # Example:
/// ```ignore
/// // 1) Define DTO:
///
/// #[DTO(response)]
/// pub struct HtmlOrJsonDataObject {
///     pub status: &'static str,
///     pub status_timestamp: u64,
/// }
///
/// // 2) Define it's HTML template:
///
/// html_format!(HtmlOrJsonDataObject, self {
///    // anything that accepts Self and returns String:
///    format!("<div><p>Status: {}.</p><p>Status timestamp: {}.</p></div>", self.status, self.status_timestamp)
/// });
///
/// // 3) Use it in a Response:
///
/// #[Response(format(html, json), default_format="html")]
/// pub enum GetHtmlOrJsonBodyResult {
///     #[Response(code = 200)]
///     Ok(HtmlOrJsonDataObject),
/// }
///
/// // 4) Return this DTO from a handler:
///
/// #[Route(method = "get", path = "/html-or-json")]
/// async fn resp_html_or_json() -> GetHtmlOrJsonBodyResult {
///     // Your handler returns raw data.
///     // Content negotiation will automatically choose JSON or HTML based on Accept header.
///     GetHtmlOrJsonBodyResult::Ok(
///         HtmlOrJsonDataObject {
///            status: "open",
///            status_timestamp: 1234567890,
///         }
///     )
/// }
/// ```
#[macro_export]
macro_rules! html_format {
    ($ty:ty, $self:ident { $template:expr }) => {
        impl ::groom::response::HtmlFormat for $ty {
            fn render($self) -> ::axum::response::Html<axum::body::Body> {
                ::axum::response::Html(
                    $template.into()
                )
            }
        }
    };
}

pub use html_format;
