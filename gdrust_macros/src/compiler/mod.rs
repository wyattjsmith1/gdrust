mod hints;
mod impl_block;
mod properties;
mod signal_args;
mod signals;

use crate::compiler::properties::{extract_properties, get_property, Property};
use crate::compiler::signals::extract_signals;
use crate::parser::gdscript_var::GdScriptVar;
use crate::Extends;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::{Parse, ParseBuffer, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{parenthesized, parse_quote, token, Expr, Field, Ident, ItemStruct, Token, Type};

pub(crate) fn compile(item: &mut ItemStruct, extends: &Extends) -> TokenStream {
    let signals = extract_signals(item);
    let properties = extract_properties(item);
    let extends_type = &extends.ty;
    item.attrs
        .push(parse_quote! { #[derive(gdnative::NativeClass)] });
    item.attrs.push(parse_quote! { #[inherit(#extends_type)]});
    item.attrs
        .push(parse_quote! { #[register_with(Self::__register_properties_and_signals)] });

    let impl_block = impl_block::impl_block(&properties, &signals, extends, item);
    quote::quote! {
        #item

        #impl_block
    }
}
