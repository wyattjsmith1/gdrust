use crate::parser::gdscript_signal;
use crate::parser::gdscript_signal::GdScriptSignal;
use crate::parser::gdscript_var::GdScriptVar;
use syn::parse::{Parse, ParseStream, Result};

#[derive(Clone)]
pub enum GdScriptItem {
    Variable(GdScriptVar),
    Signal(GdScriptSignal),
}

impl Parse for GdScriptItem {
    fn parse(input: ParseStream) -> Result<Self> {
        if gdscript_signal::next_is_signal(input) {
            Ok(GdScriptItem::Signal(input.parse::<GdScriptSignal>()?))
        } else {
            Ok(GdScriptItem::Variable(input.parse()?))
        }
    }
}
