#![forbid(unsafe_code)]
#![deny(unreachable_pub)]

use proc_macro_error::abort;

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use darling::FromMeta;

mod annotation_attrs;
mod comments;
mod controller;
mod dto;
mod http;
mod request_body;
mod response;

/// Macro to parse arguments of proc macros into structs
/// (like `default_format` part of `#[Response(default_format = "json")]`).
macro_rules! extract_macro_arguments {
    ($ty:ty, $args:expr) => {{
        match darling::ast::NestedMeta::parse_meta_list(proc_macro2::TokenStream::from(
            $args.clone(),
        )) {
            Ok(meta) => match <$ty>::from_list(&meta) {
                Ok(object_args) =>
                    Ok(object_args),

                Err(err) =>
                    Err(TokenStream::from(err.write_errors())),
            }

            Err(e) =>
                Err(TokenStream::from(darling::Error::from(e).write_errors()))
        }
    }};
}
pub(crate) use extract_macro_arguments;


/// Macro to generate `#[Controller]` implementations.
///
/// For arguments, see `controller::ControllerArgs`.
#[proc_macro_error]
#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn Controller(args: TokenStream, input: TokenStream) -> TokenStream {
    let args: proc_macro2::TokenStream = args.into();
    let controller_args = match extract_macro_arguments!(controller::ControllerArgs, &args) {
        Ok(a) => a,
        Err(e) => return e,
    };

    controller::generate(args, controller_args, input.into()).into()
}

/// Macro to generate `#[RequestBody]` implementations.
///
/// For arguments, see `request_body::RequestBodyArgs`.
#[proc_macro_error]
#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn RequestBody(args: TokenStream, input: TokenStream) -> TokenStream {
    let args: proc_macro2::TokenStream = args.into();
    let request_body_args = match extract_macro_arguments!(request_body::RequestBodyArgs, &args) {
        Ok(a) => a,
        Err(e) => return e,
    };

    request_body::generate(args, request_body_args, input.into()).into()
}

/// Macro to generate `#[Response]` implementations.
///
/// For arguments, see `response::ResponseArgs`.
#[proc_macro_error]
#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn Response(args: TokenStream, input: TokenStream) -> TokenStream {
    response::generate(args.into(), input.into()).into()
}

/// Macro to generate `#[DTO]` implementations.
///
/// For arguments, see `dto::DtoArgs`.
#[proc_macro_error]
#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn DTO(args: TokenStream, input: TokenStream) -> TokenStream {
    let args: proc_macro2::TokenStream = args.into();
    if args.is_empty() {
        abort!(args, "error in `#[DTO]` annotation: specify `request`, `response`, `parameters`, or all as DTO arguments (e.g. `#[DTO(request, response)`])")
    }

    let dto_args = match extract_macro_arguments!(dto::DtoArgs, &args) {
        Ok(a) => a,
        Err(e) => return e,
    };

    dto::generate(args, dto_args, input.into()).into()
}
