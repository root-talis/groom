/// This is a scratchpad to test out how various code pieces fit together
/// so that we can then turn them into code generation.
///
/// Nothing important here, really. This is a mess and it should be so.

use utoipa::{OpenApi, openapi, PartialSchema, ToSchema};
use utoipa::openapi::{ComponentsBuilder, ContentBuilder, OpenApiBuilder, PathsBuilder};
use utoipa::openapi::path::{OperationBuilder, ParameterBuilder, PathItemBuilder};
use utoipa::openapi::request_body::RequestBodyBuilder;
use groom_macros::DTO;
use groom::extract::GroomExtractor;
use groom::schema::GroomSchema;

#[DTO(request, response)]
struct SomeDTO {
    name: String
}

#[tokio::test]
async fn api_doc_scratchpad() {
    #[derive(OpenApi)]
    #[openapi()]
    struct ApiDoc;

    let api = OpenApiBuilder::from(ApiDoc::openapi());

    // region: should generate this for /hello-world
    let parameter = ParameterBuilder::new()
        .parameter_in(openapi::path::ParameterIn::Query)
        .name("sad")
        .schema(Some(SomeDTO::schema().1))
        .build()
    ;

    let component_schema_name = SomeDTO::schema().0;

    let components = ComponentsBuilder::new()
        .schema(component_schema_name, SomeDTO::schema().1)
        .build()
        ;

    let content = ContentBuilder::new()
        .schema(components.schemas[component_schema_name].clone())
        .build()
        ;

    let request_body = RequestBodyBuilder::new()
        .content("application/json", content)
        .build()
        ;

    let op_builder= OperationBuilder::new()
        .parameter(parameter)
        .request_body(Some(request_body))
        .description(Some("description"))
        .summary(Some("summary"));

    let op_builder = axum::extract::Path::<SomeDTO>::__openapi_modify_operation(op_builder);

    let resp = utoipa::openapi::ResponseBuilder::new().build();

    let op_builder = op_builder.response("202", resp);

    let operation = op_builder.build();

    let operation = OperationBuilder::from(operation).build();

    let path_item= PathItemBuilder::new()
        .operation(utoipa::openapi::PathItemType::Get, operation)
        .build()
        ;

    let paths = PathsBuilder::new()
        .path("/hello-world", path_item)
        .build()
        ;
    let paths = PathsBuilder::from(paths).build();

    let api = api.paths(paths);
    let api = api.components(Some(components));

    // endregion: should generate this for /hello-world

    let json = api.build().to_pretty_json().expect("expected a valid json");

    let _breakpoint = false;

    let hm = ::axum::http::header::HeaderMap::new();
    let _has_header = hm.get(::axum::http::header::ACCEPT);

    eprintln!("{}", json);
    //assert_eq!(true, false)

    let _schema = SomeDTO::schema().extract_schema();
    let _schema = String::schema().extract_schema();
}
