use proc_macro::TokenStream;
mod compiler;
mod parser;
use crate::parser::gdscript_class::GdScriptClass;
use quote::ToTokens;
use syn::parse::{Parse, ParseBuffer, ParseStream};
use syn::{parenthesized, token, ItemImpl, ItemStruct, Result, Token, Type};

mod kw {
    syn::custom_keyword!(extends);
}

pub(crate) struct Extends {
    extends: kw::extends,
    eq: Token![=],
    ty: Type,
}

impl Parse for Extends {
    fn parse(input: ParseStream) -> Result<Self> {
        let extends = input.parse()?;
        let eq = input.parse()?;
        let ty = input.parse()?;
        Ok(Self { extends, eq, ty })
    }
}

#[proc_macro_attribute]
pub fn gdrust2(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut parsed = syn::parse_macro_input!(item as ItemStruct);
    let extends = syn::parse_macro_input!(attr as Extends);
    let compiled = compiler::compile(&mut parsed, &extends);
    println!("{}", compiled.to_string());
    compiled.into()
}
