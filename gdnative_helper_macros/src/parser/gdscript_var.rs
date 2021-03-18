use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{parenthesized, Ident, Lit, Token, Type};
use syn::{token, Expr};

mod kw {
    syn::custom_keyword!(export);
    syn::custom_keyword!(or_greater);
    syn::custom_keyword!(or_lesser);
    syn::custom_keyword!(var);
    syn::custom_keyword!(no_export);
    syn::custom_keyword!(export_range);
}

#[derive(Clone)]
pub enum ExportType {
    NoHint,
    NoExport(NoExport),
    Export(Export),
    ExportRange(ExportRange),
}

#[derive(Clone)]
pub struct NoExport {
    pub at: Token![@],
    pub no_export: kw::no_export,
}

#[derive(Clone)]
pub struct Export {
    pub at: Token![@],
    pub export: kw::export,
}

#[derive(Clone)]
pub struct ExportRange {
    pub at: Token![@],
    pub export: kw::export_range,
    pub paren_token: token::Paren,
    pub range: Punctuated<Lit, Token![,]>,
}

#[derive(Clone)]
pub struct GdScriptVar {
    pub export: ExportType,
    pub var_token: kw::var,
    pub var_name: Ident,
    pub colon: Token![:],
    pub ty: Type,
    pub equals: Token![=],
    pub value: Expr,
}

impl Parse for GdScriptVar {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(GdScriptVar {
            export: input.parse()?,
            var_token: input.parse()?,
            var_name: input.parse()?,
            colon: input.parse()?,
            ty: input.parse()?,
            equals: input.parse()?,
            value: input.parse()?,
        })
    }
}

impl Parse for ExportType {
    fn parse(input: ParseStream) -> Result<Self> {
        if !input.peek(Token![@]) {
            return Ok(ExportType::NoHint);
        }
        let at = input.parse()?;
        if input.peek(kw::no_export) {
            Ok(ExportType::NoExport(NoExport {
                at,
                no_export: input.parse()?,
            }))
        } else if input.peek(kw::export) {
            Ok(ExportType::Export(Export {
                at,
                export: input.parse()?,
            }))
        } else if input.peek(kw::export_range) {
            let content;
            Ok(ExportType::ExportRange(ExportRange {
                at,
                export: input.parse()?,
                paren_token: parenthesized!(content in input),
                range: content.parse_terminated(Lit::parse)?,
            }))
        } else {
            Err(input.error("Expected an export type"))
        }
    }
}
