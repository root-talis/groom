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


#[macro_export]
macro_rules! html_format {
    ($ty:ident, $self:ident { $template:expr }) => {
        impl ::humars::response::HtmlFormat for $ty {
            fn render($self) -> ::axum::response::Html<axum::body::Body> {
                ::axum::response::Html(
                    $template.into()
                )
            }
        }
    };
}

pub use html_format;
