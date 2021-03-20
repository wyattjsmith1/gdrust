use crate::parser::gdscript_signal;
use crate::parser::gdscript_signal::GdScriptSignal;
use crate::parser::gdscript_var::GdScriptVar;
use syn::parse::{Parse, ParseStream, Result};
use crate::parser::gdscript_var;
use syn::Error;
use proc_macro2::Span;

#[derive(Clone)]
pub enum GdScriptItem {
    Variable(GdScriptVar),
    Signal(GdScriptSignal),
}

impl Parse for GdScriptItem {
    fn parse(input: ParseStream) -> Result<Self> {
        if gdscript_signal::next_is_signal(input) {
            Ok(GdScriptItem::Signal(input.parse::<GdScriptSignal>()?))
        } else if gdscript_var::next_is_var(input) {
            Ok(GdScriptItem::Variable(input.parse()?))
        } else {
            Err(Error::new(Span::call_site(), "Expected either a var or a signal"))
        }
    }
}
