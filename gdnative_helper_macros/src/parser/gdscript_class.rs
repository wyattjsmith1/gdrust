use crate::parser::gdscript_items::GdScriptItem;
use crate::parser::gdscript_var::GdScriptVar;
use syn::parse::{Parse, ParseStream};
use syn::{braced, parse_quote, Ident, Result, Type};

mod kw {
    syn::custom_keyword!(class);
    syn::custom_keyword!(extends);
}

pub struct GdScriptClass {
    pub class_token: kw::class,
    pub name: Ident,
    pub extends: Option<(kw::extends, Type)>,
    pub brace: syn::token::Brace,
    pub items: Vec<GdScriptItem>,
}

impl GdScriptClass {
    pub fn parent(&self) -> Type {
        self.extends
            .as_ref()
            .map(|(_, parent)| parent.clone())
            .unwrap_or(parse_quote! { gdnative::prelude::Object })
    }

    pub fn vars(&self) -> Vec<GdScriptVar> {
        self.items
            .iter()
            .filter_map(|x| match x {
                GdScriptItem::Variable(x) => Some(x.clone()),
                _ => None,
            })
            .collect()
    }
}

impl Parse for GdScriptClass {
    fn parse(input: ParseStream) -> Result<Self> {
        let class_token = input.parse::<kw::class>()?;
        let name = input.parse()?;
        let extends = if input.peek(kw::extends) {
            Some((input.parse()?, input.parse()?))
        } else {
            None
        };
        let content;
        let brace = braced!(content in input);
        let mut items = Vec::new();
        while !content.is_empty() {
            items.push(content.parse()?)
        }
        Ok(GdScriptClass {
            class_token,
            name,
            extends,
            brace,
            items,
        })
    }
}
