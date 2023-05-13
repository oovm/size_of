
use proc_macro2::TokenStream;
use quote::{quote, TokenStreamExt, ToTokens};
use syn::{ExprField, LitStr};
use syn::__private::TokenStream2;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
mod impl_function_name;
mod impl_name_of;


pub use self::impl_function_name::FunctionName;
pub use self::impl_name_of::NameOf;