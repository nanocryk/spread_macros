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
mod common;
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
