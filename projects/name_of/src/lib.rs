#![deny(missing_debug_implementations, missing_copy_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]

extern crate proc_macro;

mod implements;

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse_macro_input;
use implements::NameOf;
use crate::implements::FunctionName;

/// Returns the name of the item as a string.
#[proc_macro]
pub fn name_of(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as NameOf);
    TokenStream::from(item.to_token_stream())
}

/// Returns the function name of the item as a string.
#[proc_macro]
pub fn function_name(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as FunctionName);
    TokenStream::from(item.to_token_stream())
}
