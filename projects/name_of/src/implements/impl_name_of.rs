use super::*;

/// ```
///     let span = Range {
///         start: 1,
///         end: 2,
///     };
///     // start
///     println!("name: {}", name_of!(span.start));
/// ```
pub struct NameOf {
    raw: TokenStream2,
    text: LitStr,
}

impl Parse for NameOf {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        match input.parse::<ExprField>() {
            Ok(o) => {
                let raw = o.to_token_stream();
                let out = o.member;
                let text = LitStr::new(&out.to_token_stream().to_string(), out.span());
                Ok(NameOf { raw, text })
            }
            Err(_) => {
                todo!()
            }
        }
    }
}

impl ToTokens for NameOf {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let raw = &self.raw;
        let text = &self.text;
        tokens.append_all(quote! {
            {
                let _ = #raw;
                #text
            }
        });



    }
}