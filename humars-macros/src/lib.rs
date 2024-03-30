#![forbid(unsafe_code)]
#![deny(unreachable_pub)]

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;

mod attrs;
mod controller;
mod dto;
mod http;
mod response;
mod utils;

#[proc_macro_error]
#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn Controller(args: TokenStream, input: TokenStream) -> TokenStream {
    controller::generate(args.into(), input.into()).into()
}


#[proc_macro_error]
#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn Response(args: TokenStream, input: TokenStream) -> TokenStream {
    response::generate(args.into(), input.into()).into()
}


#[proc_macro_error]
#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn DTO(args: TokenStream, input: TokenStream) -> TokenStream {
    dto::generate(args.into(), input.into()).into()
}
