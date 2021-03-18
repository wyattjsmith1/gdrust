use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{parenthesized, Expr, Ident, Token, Type};

mod kw {
    syn::custom_keyword!(signal);
}

#[derive(Clone)]
pub struct SignalArg {
    pub name: Ident,
    pub colon_token: Token![:],
    pub ty: Type,
    pub default: Option<(Token![=], Expr)>,
}

#[derive(Clone)]
pub struct GdScriptSignal {
    pub signal_token: kw::signal,
    pub name: Ident,
    pub paren_token: syn::token::Paren,
    pub signal_type: Punctuated<SignalArg, Token![,]>,
}

impl Parse for GdScriptSignal {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(GdScriptSignal {
            signal_token: input.parse()?,
            name: input.parse()?,
            paren_token: parenthesized!(content in input),
            signal_type: content.parse_terminated(SignalArg::parse)?,
        })
    }
}

impl Parse for SignalArg {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse()?;
        let colon_token = input.parse()?;
        let ty = input.parse()?;
        let default = if input.peek(Token![=]) {
            Some((input.parse()?, input.parse()?))
        } else {
            None
        };
        Ok(SignalArg {
            name,
            colon_token,
            ty,
            default,
        })
    }
}

pub fn next_is_signal(input: ParseStream) -> bool {
    input.peek(kw::signal)
}
