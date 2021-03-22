use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{parenthesized, token, Expr, Ident, ItemStruct, Result, Token, Type};

#[allow(clippy::module_name_repetitions)]
pub fn extract_signals(item: &mut ItemStruct) -> Vec<SignalDecl> {
    let mut result = Vec::new();
    item.attrs = item
        .attrs
        .iter()
        .filter(|attr| {
            attr.path
                .get_ident()
                .filter(|x| *x == "signal")
                .map(|_x| {
                    let tokens: proc_macro::TokenStream = attr.tokens.clone().into();
                    let signal = syn::parse_macro_input::parse::<SignalWithParens>(tokens)
                        .expect("Could not parse signal declaration")
                        .signal;
                    result.push(signal);
                })
                .is_none()
        })
        .cloned()
        .collect();
    result
}

pub struct SignalWithParens {
    pub paren_token: token::Paren,
    pub signal: SignalDecl,
}

pub struct SignalDecl {
    pub name: Ident,
    pub paren_token: token::Paren,
    pub args: Punctuated<SignalArgDecl, Token![,]>,
}

pub struct SignalArgDecl {
    pub name: Ident,
    pub colon: Token![:],
    pub ty: Type,
    pub default: Option<(Token![=], Expr)>,
}

impl Parse for SignalWithParens {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        let paren_token = parenthesized!(content in input);
        let signal = content.parse()?;
        Ok(Self {
            paren_token,
            signal,
        })
    }
}

impl Parse for SignalDecl {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        let name = input.parse()?;
        let paren_token = parenthesized!(content in input);
        let args = content.parse_terminated(SignalArgDecl::parse)?;
        Ok(Self {
            name,
            paren_token,
            args,
        })
    }
}

impl Parse for SignalArgDecl {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse()?;
        let colon = input.parse()?;
        let ty = input.parse()?;
        let default = if input.peek(Token![=]) {
            let eq = input.parse()?;
            let value = input.parse()?;
            Some((eq, value))
        } else {
            None
        };
        Ok(Self {
            name,
            colon,
            ty,
            default,
        })
    }
}
