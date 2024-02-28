//! Proc-macro crate for [`nanotweaks`](https://crates.io/crates/nanotweaks), which you
//! should probably use instead of this one.

use {
    proc_macro2::{Span, TokenStream},
    quote::{quote, quote_spanned},
    std::fmt::Write,
    syn::{
        braced,
        parse::{Parse, ParseStream},
        parse_macro_input,
        punctuated::Punctuated,
        spanned::Spanned,
        token::Brace,
        Token,
    },
};

mod anon;
mod assert_fields_eq;
mod common;
mod fn_struct;
mod slet;
mod spread;

#[proc_macro]
pub fn spread(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    spread::spread(tokens)
}

#[proc_macro]
pub fn anon(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    anon::anon(tokens)
}

#[proc_macro]
pub fn slet(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    slet::slet(tokens)
}

#[proc_macro]
pub fn fn_struct(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    fn_struct::fn_struct(tokens)
}

#[proc_macro]
pub fn assert_fields_eq(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    assert_fields_eq::assert_fields_eq(tokens)
}
