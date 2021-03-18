use crate::parser::gdscript_var::GdScriptVar;
use syn::parse::{Parse, ParseStream, Result};

#[derive(Clone)]
pub enum GdScriptItem {
    Variable(GdScriptVar),
    // Function(syn::ItemFn),
}

impl Parse for GdScriptItem {
    fn parse(input: ParseStream) -> Result<Self> {
        // if input.peek()
        Ok(GdScriptItem::Variable(input.parse()?))
    }
}
