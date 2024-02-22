use super::*;

pub enum SpreadItem {
    Field(Field),
    SpreadList(SpreadList),
    FinalSpread(Token![..], syn::Expr),
}

pub struct Field {
    pub is_mut: Option<Token![mut]>,
    pub modifier: Option<SpreadModifier>,
    pub name: syn::Ident,
    pub value: Option<syn::Expr>,
}

pub enum SpreadModifier {
    Ref(Token![&]),
    RefMut(Token![&], Token![mut]),
    Into(Token![>]),
    Clone(Token![+]),
    CloneInto(Token![+], Token![>]),
}

pub struct SpreadList {
    pub fields_list: Punctuated<Field, Token![,]>,
    pub source: syn::Expr,
    pub source_ident: syn::Ident,
}

impl Parse for SpreadItem {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(Brace) {
            input.parse().map(SpreadItem::SpreadList)
        } else if lookahead.peek(Token![..]) {
            Ok(SpreadItem::FinalSpread(input.parse()?, input.parse()?))
        } else {
            Ok(SpreadItem::Field(input.parse()?))
        }
    }
}

impl SpreadItem {
    pub fn field_expansion(&self) -> TokenStream {
        match self {
            Self::Field(field) => match &field.value {
                Some(value) => field.field_expansion(quote! { #value }),
                None => {
                    let source = field.name.clone();
                    field.field_expansion(quote! { #source })
                }
            },
            Self::SpreadList(spread_list) => spread_list.field_expansion(),
            Self::FinalSpread(token_dotdot, source) => {
                quote! { #token_dotdot #source }
            }
        }
    }

    pub fn let_expansion(&self) -> TokenStream {
        match self {
            Self::Field(field) => {
                let source = field.name.clone();
                let is_mut = field.is_mut;
                let expansion = match &field.value {
                    Some(value) => field.value_with_modifiers(quote! { #value }),
                    None => field.value_with_modifiers(quote! { #source }),
                };
                quote!( let #is_mut #source = #expansion; )
            }
            Self::SpreadList(spread_list) => spread_list.let_expansion(),
            Self::FinalSpread(dotdot, _) => {
                syn::Error::new(dotdot.span(), "`..remaining` is not allowed in this macro")
                    .to_compile_error()
            }
        }
    }
}

impl Parse for Field {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let is_mut = {
            let lookahead = input.lookahead1();
            if lookahead.peek(Token![mut]) {
                Some(input.parse()?)
            } else {
                None
            }
        };

        let lookahead = input.lookahead1();

        let modifier = if lookahead.peek(Token![&]) {
            let token_ref = input.parse()?;

            let lookahead = input.lookahead1();

            if lookahead.peek(Token![mut]) {
                let token_mut = input.parse()?;
                Some(SpreadModifier::RefMut(token_ref, token_mut))
            } else if lookahead.peek(syn::Ident) {
                // don't parse it now
                Some(SpreadModifier::Ref(token_ref))
            } else {
                Err(lookahead.error())?
            }
        } else if lookahead.peek(Token![>]) {
            let token_into = input.parse()?;
            Some(SpreadModifier::Into(token_into))
        } else if lookahead.peek(Token![+]) {
            let token_clone = input.parse()?;

            let lookahead = input.lookahead1();

            if lookahead.peek(Token![>]) {
                let token_into = input.parse()?;
                Some(SpreadModifier::CloneInto(token_clone, token_into))
            } else if lookahead.peek(syn::Ident) {
                // don't parse it now
                Some(SpreadModifier::Clone(token_clone))
            } else {
                Err(lookahead.error())?
            }
        } else if lookahead.peek(syn::Ident) {
            // don't parse it now
            None
        } else {
            Err(lookahead.error())?
        };

        let name = input.parse()?;

        let value = {
            let lookahead = input.lookahead1();

            if lookahead.peek(Token![:]) {
                let _: Token![:] = input.parse()?;
                let value = input.parse()?;
                Some(value)
            } else {
                None
            }
        };

        Ok(Field {
            is_mut,
            modifier,
            name,
            value,
        })
    }
}

impl Field {
    fn field_expansion(&self, source: proc_macro2::TokenStream) -> TokenStream {
        let name = &self.name;
        let value_with_modifiers = self.value_with_modifiers(source);

        quote! { #name: #value_with_modifiers }.into()
    }

    fn value_with_modifiers(&self, source: proc_macro2::TokenStream) -> TokenStream {
        match self.modifier {
            Some(SpreadModifier::Ref(token_ref)) => {
                quote! { #token_ref #source }
            }
            Some(SpreadModifier::RefMut(token_ref, token_mut)) => {
                quote! { #token_ref #token_mut #source }
            }
            Some(SpreadModifier::Into(token_into)) => {
                let into = quote_spanned!(token_into.span()=> .into());
                quote! { #source #into }
            }
            Some(SpreadModifier::Clone(token_clone)) => {
                let clone = quote_spanned!(token_clone.span()=> .clone());
                quote! { #source #clone }
            }
            Some(SpreadModifier::CloneInto(token_clone, token_into)) => {
                let clone = quote_spanned!(token_clone.span()=> .clone());
                let into = quote_spanned!(token_into.span()=> .into());
                quote! { #source #clone #into }
            }
            None => quote! { #source },
        }
        .into()
    }
}

impl Parse for SpreadList {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let braced;
        braced!(braced in input);

        let fields_list = Punctuated::<Field, _>::parse_terminated(&braced)?;
        let _: Token![in] = input.parse()?;
        let source: syn::Expr = input.parse()?;

        let source_ident: String = fields_list
            .iter()
            .fold(String::from("_"), |mut buf, field| {
                write!(buf, "_{}", field.name).expect("to write String");
                buf
            });
        let source_ident = syn::Ident::new(&source_ident, source.span());

        Ok(SpreadList {
            fields_list,
            source,
            source_ident,
        })
    }
}

impl SpreadList {
    fn field_expansion(&self) -> TokenStream {
        let source = &self.source_ident;
        let fields = self.fields_list.iter().map(|field| {
            let name = &field.name;
            field.field_expansion(quote! { #source . #name })
        });

        quote! { #( #fields ),* }
    }

    fn let_expansion(&self) -> TokenStream {
        let source = &self.source;
        let fields = self.fields_list.iter().map(|field| {
            let name = &field.name;
            field.value_with_modifiers(quote! { __source . #name })
        });
        let fields_mut = self.fields_list.iter().map(|field| &field.is_mut);
        let fields_name = self.fields_list.iter().map(|field| &field.name);

        quote! {
            let (
                #( #fields_mut #fields_name , )*
            ) = {
                let __source = #source;
                ( #( #fields , )* )
            };
        }
    }
}
