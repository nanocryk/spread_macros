use super::{common::*, *};

pub fn spread(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let Spread { struct_name, items } = parse_macro_input!(tokens as Spread);

    let let_sources = items.iter().filter_map(|item| match item {
        SpreadItem::SpreadList(SpreadList {
            source,
            source_ident,
            ..
        }) => Some(quote! { let #source_ident = #source; }),
        _ => None,
    });

    let fields_expansions = items.iter().map(SpreadItem::field_expansion);

    quote! {
        {
            #( #let_sources )*

            #struct_name {
                #( #fields_expansions ),*
            }
        }
    }
    .into()
}

struct Spread {
    struct_name: syn::Ident,
    items: Punctuated<SpreadItem, Token![,]>,
}

impl Parse for Spread {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let struct_name = input.parse()?;

        let braced;
        let braces = braced!(braced in input);

        let mut items = Punctuated::<SpreadItem, Token![,]>::parse_terminated(&braced)?;

        // Forbid empty struct
        if items.is_empty() {
            return Err(syn::Error::new(
                braces.span.join(),
                "Braces cannot be empty, no need for a macro to instanciate an empty struct",
            ));
        }

        // Only allow FinalSpread as last item
        for item in items.iter().rev().skip(1) {
            if let SpreadItem::FinalSpread(dotdot, _) = item {
                return Err(syn::Error::new(
                    dotdot.span(),
                    "`..remaining` can only be used as the last item",
                ));
            }
        }

        // Cannot have trailing comma after FinalSpread
        if let Some(SpreadItem::FinalSpread(_, _)) = items.last() {
            if let Some(trailing) = items.pop_punct() {
                return Err(syn::Error::new(
                    trailing.span(),
                    "remove trailing comma after `..remaining`",
                ));
            }
        }

        // Disallow `mut` prefix
        for item in items.iter() {
            match item {
                SpreadItem::Field(Field {
                    is_mut: Some(token_mut),
                    ..
                }) => {
                    return Err(syn::Error::new(
                        token_mut.span(),
                        "`mut` prefix is not allowed in this macro",
                    ))
                }
                SpreadItem::SpreadList(list) => {
                    for field in list.fields_list.iter() {
                        if let Some(token_mut) = field.is_mut {
                            return Err(syn::Error::new(
                                token_mut.span(),
                                "`mut` prefix is not allowed in this macro",
                            ));
                        }
                    }
                }
                _ => (),
            }
        }

        Ok(Self { struct_name, items })
    }
}
