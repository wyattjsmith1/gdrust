use proc_macro::TokenStream;
mod compiler;

use syn::parse::{Parse, ParseStream};
use syn::{parse_quote, ItemStruct, Result, Token, Type};

mod kw {
    syn::custom_keyword!(extends);
}

pub(crate) struct Extends {
    ty: Type,
}

impl Parse for Extends {
    fn parse(input: ParseStream) -> Result<Self> {
        let _extends = input.parse::<kw::extends>()?;
        let _eq = input.parse::<Token![=]>()?;
        let ty = input.parse()?;
        Ok(Self { ty })
    }
}

#[proc_macro_attribute]
pub fn gdrust(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut parsed = syn::parse_macro_input!(item as ItemStruct);
    let extends = syn::parse_macro_input::parse::<Extends>(attr).unwrap_or(Extends {
        ty: parse_quote! { gdnative::api::Object },
    });
    let compiled = compiler::compile(&mut parsed, &extends);
    // println!("{}", compiled.to_string());
    compiled.into()
}
