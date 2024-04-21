#![forbid(unsafe_code)]
#![deny(unreachable_pub)]

use proc_macro_error::abort;

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use darling::FromMeta;

mod attrs;
mod controller;
mod dto;
mod http;
mod response;
mod utils;


macro_rules! parse_nested_meta {
    ($ty:ty, $args:expr) => {{
        let meta = match darling::ast::NestedMeta::parse_meta_list(proc_macro2::TokenStream::from(
            $args,
        )) {
            Ok(v) => v,
            Err(e) => {
                return TokenStream::from(darling::Error::from(e).write_errors());
            }
        };

        match <$ty>::from_list(&meta) {
            Ok(object_args) => object_args,
            Err(err) => return TokenStream::from(err.write_errors()),
        }
    }};
}

#[proc_macro_error]
#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn Controller(_args: TokenStream, input: TokenStream) -> TokenStream {
    controller::generate(input.into()).into()
}


#[proc_macro_error]
#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn Response(args: TokenStream, input: TokenStream) -> TokenStream {
    let args: proc_macro2::TokenStream = args.into();
    /*if args.is_empty() {
        abort!(args, "specify `format` as Response (e.g. `#[Response(format(plain_text))`])")
    }*/

    let response_args = parse_nested_meta!(response::ResponseArgs, args.clone());
    response::generate(args, response_args, input.into()).into()
}


#[proc_macro_error]
#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn DTO(args: TokenStream, input: TokenStream) -> TokenStream {
    let args: proc_macro2::TokenStream = args.into();
    if args.is_empty() {
        abort!(args, "specify `request`, `response` or both as DTO arguments (e.g. `#[DTO(request, response(json))`])")
    }

    let dto_args = parse_nested_meta!(dto::DtoArgs, args.clone());
    dto::generate(args, dto_args, input.into()).into()
}
