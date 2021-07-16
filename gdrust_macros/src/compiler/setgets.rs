use crate::compiler::properties::{SetGet, SetGetMethods};
use proc_macro2::TokenStream;
use syn::Expr;
use quote::{ToTokens, quote};
use proc_macro2::Ident;


pub fn property_setter(setget: &SetGet, ident: &Ident) -> TokenStream {
    let setter = match setget {
        SetGet::Set(setter) => Expr::Path(setter.method.clone()).into_token_stream(),
        SetGet::SetGet(SetGetMethods { setter, .. }) => Expr::Path(setter.clone()).into_token_stream(),
        _ => quote::quote! {|this, _owner, val| {this.#ident = val}}
    };
    quote! {.with_setter(#setter)}
}

pub fn property_getter(setget: &SetGet, ident: &Ident) -> TokenStream {
    let getter = match setget {
        SetGet::Get(getter) => Expr::Path(getter.method.clone()).into_token_stream(),
        SetGet::SetGet(SetGetMethods { getter, .. }) => Expr::Path(getter.clone()).into_token_stream(),
        _ => quote::quote! {|this, _owner|{&this.#ident}}
    };
    quote! {.with_ref_getter(#getter)}
}
