use super::{common::*, *};

pub fn anon(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let anon = parse_macro_input!(tokens as Anon);
    anon.expand().into()
}

pub struct Anon {
    pub items: Punctuated<SpreadItem, Token![,]>,
}

impl Anon {
    pub fn expand(self) -> TokenStream {
        let Self { items } = self;

        let let_sources = items.iter().filter_map(|item| match item {
            SpreadItem::SpreadList(SpreadList {
                source,
                source_ident,
                ..
            }) => Some(quote! { let #source_ident = #source; }),
            _ => None,
        });

        let fields_expansions = items.iter().map(SpreadItem::field_expansion);

        let mut fields_name = vec![];

        for item in items.iter() {
            match item {
                SpreadItem::Field(Field { name, .. }) => {
                    fields_name.push(name.clone());
                }
                SpreadItem::SpreadList(SpreadList { fields_list, .. }) => {
                    for Field { name, .. } in fields_list.iter() {
                        fields_name.push(name.clone());
                    }
                }
                SpreadItem::FinalSpread(_, _) => {
                    unreachable!("FinalSpread is not allowed in anon!")
                }
            }
        }

        let fields_type: Vec<_> = fields_name
            .iter()
            .enumerate()
            .map(|(i, _)| syn::Ident::new(&format!("T{i}"), Span::call_site()))
            .collect();

        #[cfg(feature = "serde_derive")]
        let serde_derive = Some(quote! { #[derive(serde::Serialize, serde::Deserialize)] });
        #[cfg(not(feature = "serde_derive"))]
        let serde_derive = None::<TokenStream>;

        quote! {
            {
                #[derive(Copy, Clone, Debug, PartialEq, Eq)]
                #serde_derive
                struct Anon < #( #fields_type ),* > {
                    #(
                        #fields_name: #fields_type
                    ),*
                }

                #( #let_sources )*

                Anon {
                    #( #fields_expansions ),*
                }
            }
        }
    }
}

impl Parse for Anon {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let items = Punctuated::<SpreadItem, Token![,]>::parse_terminated(input)?;

        // Forbid empty struct
        if items.is_empty() {
            return Err(syn::Error::new(
                Span::call_site(),
                "Anon struct must have at least one field",
            ));
        }

        // No `..remaining`
        for item in items.iter() {
            if let SpreadItem::FinalSpread(dotdot, _) = item {
                return Err(syn::Error::new(
                    dotdot.span(),
                    "`..remaining` is not allowed in this macro",
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

        Ok(Self { items })
    }
}
