use axum::{Extension, Router, http::StatusCode, response::IntoResponse, routing::get};
use color_eyre::eyre::Result;
use groom::router::GroomRouterValid;
use groom_macros::Controller;
use utoipa::OpenApi;

use crate::state::AppState;

pub fn make_router() -> Result<Router> {
    let groom_router = controller::into_router()
        .validate()
        .expect("GroomRouter validation failed for htmx controller")
    ;

    let spec = make_spec(&groom_router)?;

    Ok(groom_router
        .to_axum_router()
        .route("/spec.yaml", get(get_spec))
        .layer(Extension(spec))
        .with_state(AppState::new()))
}

#[Controller(state_type = AppState)]
mod controller {
    use axum::{extract::State, response::IntoResponse};

    use groom::{
        extract::GroomExtractor,
        html_format,
        response::Response,
    };
    use groom_macros::{
        DTO,
        RequestBody,
        Response
    };

    use crate::{
        state::{AppState, MessageOfTheDay},
        template::render_template,
    };

    // region: handlers
    //

    #[Route(method = "get", path = "/")]
    pub async fn index(State(state): State<AppState>) -> PageView {
        let mut motd = state.motd.lock().expect("message mutex should not be poisoned");
        motd.shown = motd.shown.saturating_add(1);
        let (message, shown) = read_motd(&motd);
        PageView { message, shown }
    }

    #[Route(method = "put", path = "/message")]
    pub async fn update_message(
        State(state): State<AppState>,
        body: UpdateMessageRequest,
    ) -> Result<MessageView, UpdateMessageError> {
        let trimmed = body.message.trim();

        if trimmed.starts_with('/') {
            return Err(UpdateMessageError::BadArgument(ValidationErrorView {
                text: "New message starts with a slash symbol.",
            }));
        }

        let char_count = trimmed.chars().count();

        if char_count <= 3 {
            return Err(UpdateMessageError::BadArgument(ValidationErrorView {
                text: "New message is too short.",
            }));
        }

        if char_count >= 512 {
            return Err(UpdateMessageError::BadArgument(ValidationErrorView {
                text: "New message is too long.",
            }));
        }

        let mut motd = state.motd.lock().expect("message mutex should not be poisoned");
        motd.message = trimmed.to_owned();
        motd.shown = 0;
        let (message, shown) = read_motd(&motd);

        Ok(MessageView { message, shown })
    }

    //
    // endregion: handlers

    // region: view models
    //

    #[Response(format(html))]
    pub struct PageView {
        pub message: String,
        pub shown: u32,
    }

    html_format!(PageView, self {
        render_template(
            "page.jinja",
            minijinja::context! {
                message => self.message,
                shown => self.shown,
            },
        )
    });

    #[Response(format(html))]
    pub struct MessageView {
        pub message: String,
        pub shown: u32,
    }

    html_format!(MessageView, self {
        render_template(
            "message_block.jinja",
            minijinja::context! {
                message => self.message,
                shown => self.shown,
            },
        )
    });

    #[Response(format(html))]
    pub struct ValidationErrorView {
        pub text: &'static str,
    }

    html_format!(ValidationErrorView, self {
        render_template(
            "error.jinja",
            minijinja::context! {
                text => self.text,
            },
        )
    });

    #[Response(format(html))]
    pub enum UpdateMessageError {
        #[Response(code = 400)]
        BadArgument(ValidationErrorView),
    }

    #[RequestBody(format(url_encoded))]
    pub struct UpdateMessageRequest {
        message: String,
    }

    //
    // endregion: view models

    // region: utils
    //

    fn read_motd(motd: &MessageOfTheDay) -> (String, u32) {
        (motd.message.clone(), motd.shown)
    }

    //
    // endregion: utils
}

#[derive(Clone)]
struct Spec(String);

fn make_spec(r: &GroomRouterValid<AppState>) -> Result<Spec> {
    #[derive(utoipa::OpenApi)]
    #[openapi(
        info(
            title = "HTMX example (Groom)",
            description = "Message-of-the-day page with HTMX updates and Minijinja templates",
            version = "0.0.1",
            contact(name = "name", email = "mail@example.com")
        )
    )]
    struct ApiDoc;

    Ok(Spec(r.to_openapi(ApiDoc::openapi()).to_yaml()?))
}

async fn get_spec(Extension(Spec(spec)): Extension<Spec>) -> impl IntoResponse {
    (StatusCode::OK, spec)
}
