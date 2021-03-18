use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{parenthesized, Ident, Lit, LitStr, Token, Type};
use syn::{token, Expr};

mod kw {
    syn::custom_keyword!(export);
    syn::custom_keyword!(or_greater);
    syn::custom_keyword!(or_lesser);
    syn::custom_keyword!(var);
    syn::custom_keyword!(no_export);
    syn::custom_keyword!(export_range);
    syn::custom_keyword!(export_enum);
    syn::custom_keyword!(export_file);
    syn::custom_keyword!(export_dir);
    syn::custom_keyword!(export_global_file);
    syn::custom_keyword!(export_global_dir);
    syn::custom_keyword!(export_multiline);
    syn::custom_keyword!(export_exp_range);
    syn::custom_keyword!(export_color_no_alpha);
}

#[derive(Clone)]
pub enum ExportType {
    NoHint,
    NoExport(NoExport),
    Export(Export),
    ExportRange(ExportRange),
    ExportExpRange(ExportExpRange),
    ExportEnum(ExportEnum),
    ExportFile(ExportFile),
    ExportDir(ExportDir),
    ExportGlobalFile(ExportGlobalFile),
    ExportGlobalDir(ExportGlobalDir),
    ExportMultiline(ExportMultiline),
    ExportColorNoAlpha(ExportColorNoAlpha),
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
pub struct ExportExpRange {
    pub at: Token![@],
    pub export: kw::export_exp_range,
    pub paren_token: token::Paren,
    pub range: Punctuated<Lit, Token![,]>,
}

#[derive(Clone)]
pub struct ExportEnum {
    pub at: Token![@],
    pub export: kw::export_enum,
    pub paren_token: token::Paren,
    pub values: Punctuated<LitStr, Token![,]>,
}

#[derive(Clone)]
pub struct ExportFile {
    pub at: Token![@],
    pub export_file: kw::export_file,
    pub filter: Option<(token::Paren, LitStr)>,
}

#[derive(Clone)]
pub struct ExportGlobalFile {
    pub at: Token![@],
    pub export_global_file: kw::export_global_file,
    pub filter: Option<(token::Paren, LitStr)>,
}

#[derive(Clone)]
pub struct ExportDir {
    pub at: Token![@],
    pub export_dir: kw::export_dir,
}

#[derive(Clone)]
pub struct ExportGlobalDir {
    pub at: Token![@],
    pub export_dir: kw::export_global_dir,
}

#[derive(Clone)]
pub struct ExportMultiline {
    pub at: Token![@],
    pub export_multiline: kw::export_multiline,
}

#[derive(Clone)]
pub struct ExportColorNoAlpha {
    pub at: Token![@],
    pub export_color_no_alpha: kw::export_color_no_alpha,
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
        } else if input.peek(kw::export_exp_range) {
            let content;
            Ok(ExportType::ExportExpRange(ExportExpRange {
                at,
                export: input.parse()?,
                paren_token: parenthesized!(content in input),
                range: content.parse_terminated(Lit::parse)?,
            }))
        } else if input.peek(kw::export_enum) {
            let content;
            Ok(ExportType::ExportEnum(ExportEnum {
                at,
                export: input.parse()?,
                paren_token: parenthesized!(content in input),
                values: content.parse_terminated(<LitStr as Parse>::parse)?,
            }))
        } else if input.peek(kw::export_file) {
            let export_file = input.parse()?;
            let filter = if input.peek(token::Paren) {
                let contents;
                Some((parenthesized!(contents in input), contents.parse()?))
            } else {
                None
            };
            Ok(ExportType::ExportFile(ExportFile {
                at,
                export_file,
                filter,
            }))
        } else if input.peek(kw::export_dir) {
            Ok(ExportType::ExportDir(ExportDir {
                at,
                export_dir: input.parse()?,
            }))
        } else if input.peek(kw::export_global_file) {
            let export_global_file = input.parse()?;
            let filter = if input.peek(token::Paren) {
                let contents;
                Some((parenthesized!(contents in input), contents.parse()?))
            } else {
                None
            };
            Ok(ExportType::ExportGlobalFile(ExportGlobalFile {
                at,
                export_global_file,
                filter,
            }))
        } else if input.peek(kw::export_global_dir) {
            Ok(ExportType::ExportGlobalDir(ExportGlobalDir {
                at,
                export_dir: input.parse()?,
            }))
        } else if input.peek(kw::export_multiline) {
            Ok(ExportType::ExportMultiline(ExportMultiline {
                at,
                export_multiline: input.parse()?,
            }))
        } else if input.peek(kw::export_color_no_alpha) {
            Ok(ExportType::ExportColorNoAlpha(ExportColorNoAlpha {
                at,
                export_color_no_alpha: input.parse()?,
            }))
        } else {
            Err(input.error("Expected a valid export type"))
        }
    }
}
