use {
    super::{common::*, *},
    std::collections::VecDeque,
    syn::{ext::IdentExt, parenthesized},
};

pub fn fn_struct(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let FnStruct {
        struct_attr,
        vis,
        call_by_ref,
        struct_name,
        struct_gen,
        call_gen,
        fn_path,
        fields,
        return_type,
        self_,
        impl_default,
    } = parse_macro_input!(tokens as FnStruct);

    let (struct_impl_gen, struct_ty_gen, struct_where) = struct_gen.split_for_impl();
    let (call_impl_gen, _call_ty_gen, call_where) = call_gen.split_for_impl();

    let fields_name: Vec<_> = fields.iter().map(|field| &field.name).collect();
    let fields_type: Vec<_> = fields.iter().map(|field| &field.type_).collect();
    let fields_value: Vec<_> = fields
        .iter()
        .map(|field| {
            let source = field.name.clone();
            Field::from(field.clone()).value_with_modifiers(quote! { self . #source })
        })
        .collect();

    // We generate `-> ()` so that error message can provided expected type
    let return_type = if let Some(rt) = return_type {
        quote! { -> #rt }
    } else {
        quote! { -> () }
    };

    let impl_default = if impl_default {
        let fields_default_value: Vec<_> = fields.iter().map(|field| &field.value).collect();

        Some(quote! {
            impl #struct_impl_gen ::core::default::Default for #struct_name #struct_ty_gen #struct_where {
                fn default() -> Self {
                    Self {
                        #( #fields_name: #fields_default_value ),*
                    }
                }
            }
        })
    } else {
        None
    };

    let (self_in_arg, self_out_arg) = if let Some(TypedField { modifier, name, .. }) = &self_ {
        let modifier = match modifier {
            Some(SpreadModifier::Ref(token_ref)) => quote! { #token_ref },
            Some(SpreadModifier::RefMut(token_ref, token_mut)) => quote! { #token_ref #token_mut},
            None => quote! {},
            _ => {
                return syn::Error::new(
                    name.span(),
                    "only `&`, `&mut` or no modifier is allows before `self`",
                )
                .into_compile_error()
                .into()
            }
        };

        let mut self_type = fn_path.clone();

        // Fully Qualified Path `<T as Trait>::Item`, we need to turn it into just
        // `T`.
        if let Some(syn::QSelf { ty, .. }) = &self_type.qself {
            (
                Some(quote! { __self: #modifier #ty , }),
                Some(quote! { __self, }),
            )
        }
        // Otherwise this is a normal path to a method in a type, so we simply have
        // to remove the last item: the method part.
        // Note that if it is a free standing function it will qualify the module containing
        // this function, which is not a valid type.
        else {
            if self_type.path.segments.pop().is_none() {
                return syn::Error::new(
                    fn_path.span(),
                    "Cannot use `self` with a function that is not a method",
                )
                .into_compile_error()
                .into();
            }
            self_type.path.segments.pop_punct();

            (
                Some(quote! { __self: #modifier #self_type , }),
                Some(quote! { __self, }),
            )
        }
    } else {
        (None, None)
    };

    quote! {
        #( #struct_attr )*
        #vis struct #struct_name #struct_ty_gen {
            #( #fields_name: #fields_type ),*
        }

        #impl_default

        impl #struct_impl_gen #struct_name #struct_ty_gen #struct_where {
            pub fn call #call_impl_gen ( #call_by_ref self, #self_in_arg) #return_type #call_where {
                #fn_path ( #self_out_arg #( #fields_value ),*  )
            }
        }
    }
    .into()
}

struct FnStruct {
    struct_attr: Vec<syn::Attribute>,
    vis: syn::Visibility,
    call_by_ref: Option<Token![&]>,
    struct_name: syn::Ident,
    struct_gen: syn::Generics,
    call_gen: syn::Generics,
    fn_path: syn::ExprPath,
    fields: VecDeque<TypedField>,
    return_type: Option<syn::Type>,
    self_: Option<TypedField>,
    impl_default: bool,
}

impl Parse for FnStruct {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let struct_attr = input.call(syn::Attribute::parse_outer)?;

        let vis = input.parse()?;

        let lookahead = input.lookahead1();
        let call_by_ref = if lookahead.peek(Token![&]) {
            Some(input.parse()?)
        } else {
            None
        };

        let struct_name = input.parse()?;
        let mut struct_gen: syn::Generics = input.parse()?;

        let lookahead = input.lookahead1();
        if lookahead.peek(Token![where]) {
            struct_gen.where_clause = Some(input.parse()?);
        }

        let _for: Token![for] = input.parse()?;

        let mut call_gen: syn::Generics = input.parse()?;

        let lookahead = input.lookahead1();
        if lookahead.peek(Token![where]) {
            call_gen.where_clause = Some(input.parse()?);
        }

        let _: Token![fn] = input.parse()?;

        let fn_path = input.parse()?;

        let paren;
        parenthesized!(paren in input);

        let lookahead = input.lookahead1();
        let return_type = if lookahead.peek(Token![->]) {
            let _: Token![->] = input.parse()?;
            Some(input.parse()?)
        } else {
            None
        };

        let mut fields: VecDeque<_> =
            Punctuated::<TypedField, Token![,]>::parse_terminated(&paren)?
                .into_iter()
                .collect();

        // Extract initial self if any.
        let self_ = if let Some(first) = fields.front() {
            if first.type_.is_none() {
                fields.pop_front()
            } else {
                None
            }
        } else {
            None
        };

        // Forbid other self
        for field in &fields {
            if field.type_.is_none() {
                return Err(syn::Error::new(
                    field.name.span(),
                    "`self` is only allowed once in first position",
                ));
            }
        }

        // Fields should either all have values or none.
        let have_value_count = fields.iter().filter(|field| field.value.is_some()).count();
        if have_value_count != 0 && have_value_count != fields.len() {
            return Err(syn::Error::new(
                Span::call_site(),
                "Fields must either all have values (`= value`) or none have",
            ));
        }

        Ok(FnStruct {
            struct_attr,
            vis,
            call_by_ref,
            struct_name,
            struct_gen,
            call_gen,
            fn_path,
            fields,
            return_type,
            self_,
            impl_default: have_value_count > 0,
        })
    }
}

#[derive(Clone)]
pub struct TypedField {
    pub modifier: Option<SpreadModifier>,
    pub name: syn::Ident,
    // None = Self
    pub type_: Option<syn::Type>,
    pub value: Option<syn::Expr>,
}

impl From<TypedField> for Field {
    fn from(value: TypedField) -> Field {
        let TypedField {
            modifier,
            name,
            value,
            ..
        } = value;
        Field {
            modifier,
            name,
            is_mut: None,
            value,
        }
    }
}

impl Parse for TypedField {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let modifier = SpreadModifier::parse(input)?;
        let name = input.call(syn::Ident::parse_any)?;

        if &name.to_string() == "self" {
            if matches!(
                &modifier,
                Some(
                    SpreadModifier::Into(_)
                        | SpreadModifier::Clone(_)
                        | SpreadModifier::CloneInto(_, _)
                )
            ) {
                return Err(syn::Error::new(
                    name.span(),
                    "only `&`, `&mut` or no modifier is allows before `self`",
                ));
            }

            Ok(Self {
                modifier,
                name,
                type_: None,
                value: None,
            })
        } else {
            let _: Token![:] = input.parse()?;
            let type_ = Some(input.parse()?);

            let value = {
                let lookahead = input.lookahead1();

                if lookahead.peek(Token![=]) {
                    let _: Token![=] = input.parse()?;
                    let value = input.parse()?;
                    Some(value)
                } else {
                    None
                }
            };

            Ok(Self {
                modifier,
                name,
                type_,
                value,
            })
        }
    }
}
