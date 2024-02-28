use super::{common::*, *};

pub fn slet(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let SLet { items } = parse_macro_input!(tokens as SLet);

    let let_expansions = items.iter().map(SpreadItem::let_expansion);

    quote! {
        #( #let_expansions )*
    }
    .into()
}

struct SLet {
    items: Punctuated<SpreadItem, Token![,]>,
}

impl Parse for SLet {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let items = Punctuated::<SpreadItem, Token![,]>::parse_terminated(input)?;

        // Forbid empty struct
        if items.is_empty() {
            return Err(syn::Error::new(
                Span::call_site(),
                "Must list at least one identifier",
            ));
        }

        // No `..remaining` or `field: value`
        for item in items.iter() {
            if let SpreadItem::FinalSpread(dotdot, _) = item {
                return Err(syn::Error::new(
                    dotdot.span(),
                    "`..remaining` is not allowed in this macro",
                ));
            }
        }

        Ok(Self { items })
    }
}
