use syn::{parse::Parse, Ident, Token};

pub(crate) struct TestContextArgs {
    pub(crate) context_type: Ident,
    pub(crate) skip_teardown: bool,
}

impl Parse for TestContextArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut skip_teardown = false;
        let mut context_type: Option<Ident> = None;

        while !input.is_empty() {
            let lookahead = input.lookahead1();
            if lookahead.peek(kw::skip_teardown) {
                if skip_teardown {
                    return Err(input.error("expected only a single `skip_teardown` argument"));
                }
                let _ = input.parse::<kw::skip_teardown>()?;
                skip_teardown = true;
            } else if lookahead.peek(Ident) {
                if context_type.is_some() {
                    return Err(input.error("expected only a single type identifier"));
                }
                context_type = Some(input.parse()?);
            } else if lookahead.peek(Token![,]) {
                let _ = input.parse::<Token![,]>()?;
            } else {
                return Err(lookahead.error());
            }
        }

        Ok(TestContextArgs {
            context_type: context_type
                .ok_or(input.error("expected at least one type identifier"))?,
            skip_teardown,
        })
    }
}

mod kw {
    syn::custom_keyword!(skip_teardown);
}
