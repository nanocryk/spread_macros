use {
    crate::{common::*, *},
    syn::bracketed,
};

pub fn assert_fields_eq(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let assert_fields_eq = parse_macro_input!(tokens as AssertFieldsEq);

    match assert_fields_eq {
        AssertFieldsEq::List {
            left,
            right,
            fields,
            fmt_args,
        } => {
            let fields: Vec<_> = fields.into_iter().collect();
            quote! {
                {
                    #[allow(non_camel_case_types)]
                    #[derive(Debug, PartialEq, Eq)]
                    struct Fields
                    <
                        'a,
                        #( #fields, )*
                    > {
                        #(#fields: &'a #fields,)*
                    }

                    let left = &#left;
                    let left = Fields {
                        #( #fields: & (left . #fields) ,)*
                    };

                    let right = &#right;
                    let right = Fields {
                        #( #fields: & (right . #fields) ,)*
                    };

                    assert_eq!(left, right #fmt_args);
                }
            }
            .into()
        }
        AssertFieldsEq::Anon {
            left,
            anon,
            fmt_args,
        } => {
            let mut fields = vec![];

            for field in &anon.items {
                match field {
                    SpreadItem::Field(Field { name, .. }) => fields.push(name.clone()),
                    SpreadItem::SpreadList(list) => {
                        for field in list.fields_list.iter() {
                            fields.push(field.name.clone())
                        }
                    }
                    SpreadItem::FinalSpread(_, _) => {
                        unreachable!("FinalSpread is not allowed in anon!")
                    }
                }
            }

            let anon = anon.expand();

            quote! {
                {
                    let right = #anon;

                    #[allow(non_camel_case_types)]
                    #[derive(Debug, PartialEq, Eq)]
                    struct Fields
                    <
                        'a,
                        #( #fields, )*
                    > {
                        #(#fields: &'a #fields,)*
                    }

                    let left = &#left;
                    let left = Fields {
                        #( #fields: & (left . #fields) ,)*
                    };

                    let right = &right;
                    let right = Fields {
                        #( #fields: & (right . #fields) ,)*
                    };

                    assert_eq!(left, right #fmt_args);
                }
            }
            .into()
        }
    }
}

enum AssertFieldsEq {
    List {
        left: syn::Expr,
        right: syn::Expr,
        fields: Punctuated<syn::Ident, Token![,]>,
        fmt_args: TokenStream,
    },
    Anon {
        left: syn::Expr,
        anon: crate::anon::Anon,
        fmt_args: TokenStream,
    },
}

impl Parse for AssertFieldsEq {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let left = input.parse()?;
        let _: Token![,] = input.parse()?;

        let lookahead = input.lookahead1();
        if lookahead.peek(Brace) {
            let braced;
            braced!(braced in input);

            let anon = braced.parse()?;
            let fmt_args = input.parse()?;

            Ok(AssertFieldsEq::Anon {
                left,
                anon,
                fmt_args,
            })
        } else if lookahead.peek(syn::Ident) {
            let right = input.parse()?;
            let _: Token![,] = input.parse()?;

            let bracketed;
            let bracket = bracketed!(bracketed in input);

            let fields = Punctuated::parse_terminated(&bracketed)?;

            if fields.is_empty() {
                return Err(syn::Error::new(
                    bracket.span.join(),
                    "`Fields list cannot be empty",
                ));
            }

            let fmt_args = input.parse()?;

            Ok(AssertFieldsEq::List {
                left,
                right,
                fields,
                fmt_args,
            })
        } else {
            Err(lookahead.error())?
        }
    }
}
