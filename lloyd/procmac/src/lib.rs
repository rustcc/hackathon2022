#![allow(
    clippy::blocks_in_if_conditions,
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::manual_map,
    clippy::map_unwrap_or,
    clippy::needless_pass_by_value,
    clippy::option_if_let_else,
    clippy::range_plus_one,
    clippy::single_match_else,
    clippy::too_many_lines,
    clippy::wrong_self_convention
)]

extern crate proc_macro;

mod ast;
mod attr;
mod expand;
mod fmt;
mod generics;
mod tools;

use proc_macro::TokenStream;
use syn::{parse_macro_input, AttributeArgs, DeriveInput, ItemFn};

#[proc_macro_derive(EnumFrom, attributes(note))]
pub fn enumerable(input: TokenStream) -> TokenStream {
    expand::enumer::enumerable(input)
}

#[proc_macro_derive(AppConfig, attributes(profiles))]
pub fn configration(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand::config::derive(&input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

#[proc_macro_derive(EnumProp, attributes(prop))]
pub fn enum_prop(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand::enumprop::derive(&input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

#[proc_macro_derive(HttpCode, attributes(from, code))]
pub fn derive_error(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand::httpcode::derive(&input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

#[proc_macro_derive(Validator, attributes(validate))]
pub fn derive_validator(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand::validator::derive(&input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

#[proc_macro_derive(ApiModel, attributes(note, serde, validate))]
pub fn derive_apidoc(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand::apidoc::derive(&input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

#[proc_macro_derive(Component, attributes(extension))]
pub fn derive_component(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand::component::derive(&input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

#[proc_macro_attribute]
pub fn post(attr: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);
    let mut item_fn = parse_macro_input!(input as ItemFn);
    expand::apidoc::attribute(args, &mut item_fn, "post".into())
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

#[proc_macro_attribute]
pub fn get(attr: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);
    let mut item_fn = parse_macro_input!(input as ItemFn);
    expand::apidoc::attribute(args, &mut item_fn, "get".into())
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

#[proc_macro_derive(FromMultipart, attributes(serde, method))]
pub fn derive_from_multipart(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand::multipart::derive(&input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
