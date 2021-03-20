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
    syn::custom_keyword!(export_node_path);
    syn::custom_keyword!(export_flags);
    syn::custom_keyword!(export_flags_2d_physics);
    syn::custom_keyword!(export_flags_2d_render);
    syn::custom_keyword!(export_flags_3d_physics);
    syn::custom_keyword!(export_flags_3d_render);
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
    ExportNodePath(ExportNodePath),
    ExportFlags(ExportFlags),
    ExportFlags2dPhysics(ExportFlags2dPhysics),
    ExportFlags2dRender(ExportFlags2dRender),
    ExportFlags3dPhysics(ExportFlags3dPhysics),
    ExportFlags3dRender(ExportFlags3dRender),
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
pub struct ExportNodePath {
    pub at: Token![@],
    pub export_color_no_alpha: kw::export_node_path,
    pub paren_token: token::Paren,
    pub types: Punctuated<Type, Token![,]>,
}

#[derive(Clone)]
pub struct ExportFlags {
    pub at: Token![@],
    pub export_flags: kw::export_flags,
    pub paren_token: token::Paren,
    pub values: Punctuated<LitStr, Token![,]>,
}

#[derive(Clone)]
pub struct ExportFlags2dPhysics {
    pub at: Token![@],
    pub export_flags_2d_physics: kw::export_flags_2d_physics,
}

#[derive(Clone)]
pub struct ExportFlags2dRender {
    pub at: Token![@],
    pub export_flags_2d_render: kw::export_flags_2d_render,
}

#[derive(Clone)]
pub struct ExportFlags3dPhysics {
    pub at: Token![@],
    pub export_flags_3d_physics: kw::export_flags_3d_physics,
}

#[derive(Clone)]
pub struct ExportFlags3dRender {
    pub at: Token![@],
    pub export_flags_3d_render: kw::export_flags_3d_render,
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
    #[allow(clippy::too_many_lines)]
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
            let export = input.parse()?;
            let content;
            let paren_token = parenthesized!(content in input);
            let range = content.parse_terminated(Lit::parse)?;
            Ok(ExportType::ExportRange(ExportRange {
                at,
                export,
                paren_token,
                range,
            }))
        } else if input.peek(kw::export_exp_range) {
            let export = input.parse()?;
            let content;
            let paren_token = parenthesized!(content in input);
            let range = content.parse_terminated(Lit::parse)?;
            Ok(ExportType::ExportExpRange(ExportExpRange {
                at,
                export,
                paren_token,
                range,
            }))
        } else if input.peek(kw::export_enum) {
            let export = input.parse()?;
            let content;
            let paren_token = parenthesized!(content in input);
            let values = content.parse_terminated(<LitStr as Parse>::parse)?;
            Ok(ExportType::ExportEnum(ExportEnum {
                at,
                export,
                paren_token,
                values,
            }))
        } else if input.peek(kw::export_file) {
            let export_file = input.parse()?;
            let filter = if input.peek(token::Paren) {
                let contents;
                let paren = parenthesized!(contents in input);
                let hint = contents.parse()?;
                Some((paren, hint))
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
                let paren = parenthesized!(contents in input);
                let hint = contents.parse()?;
                Some((paren, hint))
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
        } else if input.peek(kw::export_node_path) {
            let contents;
            let export_color_no_alpha = input.parse()?;
            let paren_token = parenthesized!(contents in input);
            let types = contents.parse_terminated(Type::parse)?;
            Ok(ExportType::ExportNodePath(ExportNodePath {
                at,
                export_color_no_alpha,
                paren_token,
                types,
            }))
        } else if input.peek(kw::export_flags) {
            let contents;
            let export_flags = input.parse()?;
            let paren_token = parenthesized!(contents in input);
            let values = contents.parse_terminated(<LitStr as Parse>::parse)?;
            Ok(ExportType::ExportFlags(ExportFlags {
                at,
                export_flags,
                paren_token,
                values,
            }))
        } else if input.peek(kw::export_flags_2d_physics) {
            Ok(ExportType::ExportFlags2dPhysics(ExportFlags2dPhysics {
                at,
                export_flags_2d_physics: input.parse()?,
            }))
        } else if input.peek(kw::export_flags_2d_render) {
            Ok(ExportType::ExportFlags2dRender(ExportFlags2dRender {
                at,
                export_flags_2d_render: input.parse()?,
            }))
        } else if input.peek(kw::export_flags_3d_physics) {
            Ok(ExportType::ExportFlags3dPhysics(ExportFlags3dPhysics {
                at,
                export_flags_3d_physics: input.parse()?,
            }))
        } else if input.peek(kw::export_flags_3d_render) {
            Ok(ExportType::ExportFlags3dRender(ExportFlags3dRender {
                at,
                export_flags_3d_render: input.parse()?,
            }))
        } else {
            Err(input.error("Expected a valid export type"))
        }
    }
}

pub(crate) fn next_is_var(input: ParseStream) -> bool {
    input.peek(Token![@]) || input.peek(kw::var)
}
