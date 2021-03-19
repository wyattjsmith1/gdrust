use crate::parser::gdscript_items::GdScriptItem;
use crate::parser::gdscript_signal::GdScriptSignal;
use crate::parser::gdscript_var::GdScriptVar;
use syn::parse::{Parse, ParseStream};
use syn::{parse_quote, Attribute, Ident, Result, Type};

mod kw {
    syn::custom_keyword!(class);
    syn::custom_keyword!(extends);
}

pub struct GdScriptClass {
    pub attributes: Vec<Attribute>,
    pub class_token: kw::class,
    pub name: Ident,
    pub extends: Option<(kw::extends, Type)>,
    pub items: Vec<GdScriptItem>,
}

impl GdScriptClass {
    #[allow(clippy::or_fun_call)]
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

    pub fn signals(&self) -> Vec<GdScriptSignal> {
        self.items
            .iter()
            .filter_map(|x| match x {
                GdScriptItem::Signal(x) => Some(x.clone()),
                _ => None,
            })
            .collect()
    }
}

impl Parse for GdScriptClass {
    fn parse(input: ParseStream) -> Result<Self> {
        let attributes = input.call(Attribute::parse_outer)?;
        let class_token = input.parse::<kw::class>()?;
        let name = input.parse()?;
        let extends = if input.peek(kw::extends) {
            Some((input.parse()?, input.parse()?))
        } else {
            None
        };
        let mut items = Vec::new();
        while !input.is_empty() {
            items.push(input.parse()?)
        }
        Ok(GdScriptClass {
            attributes,
            class_token,
            name,
            extends,
            items,
        })
    }
}
