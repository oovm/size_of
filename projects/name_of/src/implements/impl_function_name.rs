use super::*;

/// ```
///     let span = Range {
///         start: 1,
///         end: 2,
///     };
///     // start
///     println!("name: {}", name_of!(span.start));
/// ```
pub struct FunctionName {
    level: usize,
}

impl Parse for FunctionName {
    fn parse(_: ParseStream) -> syn::Result<Self> {
        Ok(FunctionName {
            level: 0,
        })
    }
}

impl ToTokens for FunctionName {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(quote! {
            {
                fn f() {}
                fn type_name_of<T>(_: T) -> &'static str {
                    std::any::type_name::<T>()
                }
                type_name_of(f)
                    .rsplit("::")
                    .find(|&part| part != "f" && part != "{{closure}}")
                    .expect("Short function name")
            }
        });
    }
}