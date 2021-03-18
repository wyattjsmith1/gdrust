use crate::parser::gdscript_var::{
    ExportEnum, ExportExpRange, ExportFile, ExportGlobalFile, ExportRange, ExportType,
};
use proc_macro2::TokenStream;
use syn::{parse_quote, Lit, Type};

pub(crate) fn property_hint(export: &ExportType, ty: &Type) -> TokenStream {
    match export {
        ExportType::NoHint | ExportType::Export(_) => quote::quote! {},
        ExportType::NoExport(_) => {
            panic!("Should only call property_hint if there is an export. Found NoExport")
        }
        ExportType::ExportRange(export_range) => export_range_hint(export_range, ty),
        ExportType::ExportExpRange(exp_range) => export_exp_range_hint(exp_range, ty),
        ExportType::ExportEnum(export_enum) => export_enum_hint(export_enum, ty),
        ExportType::ExportFile(file) => export_file_hint(file),
        ExportType::ExportDir(_) => export_dir_hint(),
        ExportType::ExportGlobalFile(global_file) => export_global_file_hint(global_file),
        ExportType::ExportGlobalDir(_) => export_global_dir_hint(),
        ExportType::ExportMultiline(_) => export_multiline_hint(),
        ExportType::ExportColorNoAlpha(_) => export_color_no_alpha_hint(),
    }
}

fn export_exp_range_hint(range: &ExportExpRange, ty: &Type) -> TokenStream {
    export_range_hint_helper(range.range.iter().collect(), ty, true)
}

fn export_range_hint(range: &ExportRange, ty: &Type) -> TokenStream {
    export_range_hint_helper(range.range.iter().collect(), ty, false)
}

fn export_range_hint_helper(range: Vec<&Lit>, ty: &Type, is_exp: bool) -> TokenStream {
    assert!(is_number(ty), "Export range must be a number (int, float)");
    assert!(
        range.len() >= 2,
        "Export range must contain at least a min and a max"
    );
    let min = &range[0];
    let max = &range[1];
    let mut step = TokenStream::default();
    let mut or_lesser = TokenStream::default();
    let mut or_greater = TokenStream::default();
    let mut current_index = 2;
    while current_index < range.len() {
        match &range[current_index] {
            Lit::Int(int) => step = quote::quote! { .with_step(#int)},
            Lit::Float(float) => step = quote::quote! { .with_step(#float)},
            Lit::Str(str) => match str.value().as_str() {
                "or_lesser" => or_lesser = quote::quote! { .or_lesser() },
                "or_greater" => or_greater = quote::quote! { .or_greater() },
                _ => panic!("Unexpected string literal. Expected \"or_lesser\" or \"or_greater\""),
            },
            _ => panic!("Unexpected item in range: {:?}", &range[current_index]),
        }
        current_index += 1;
    }
    let range_type = if is_exp {
        quote::quote! { ExpRange }
    } else {
        quote::quote! { Range }
    };
    if is_float(ty) {
        quote::quote! {
            .with_hint(gdnative::nativescript::init::property::FloatHint::#range_type(
                gdnative::nativescript::property::RangeHint::new(#min, #max)
                    #step
                    #or_lesser
                    #or_greater
            ))
        }
    } else {
        quote::quote! {
            .with_hint(gdnative::nativescript::init::property::IntHint::#range_type(
                gdnative::nativescript::property::RangeHint::new(#min, #max)
                    #step
                    #or_lesser
                    #or_greater
            ))
        }
    }
}

fn export_enum_hint(export_enum: &ExportEnum, ty: &Type) -> TokenStream {
    let items = export_enum.values.iter();
    let hint = quote::quote! {
        gdnative::nativescript::init::property::EnumHint::new(
            vec![
                #(#items.into(),)*
            ]
        )
    };
    if is_number(ty) {
        quote::quote! {
            .with_hint(gdnative::nativescript::init::property::IntHint::Enum(#hint))
        }
    } else {
        quote::quote! {
            .with_hint(gdnative::nativescript::init::property::StringHint::Enum(#hint))
        }
    }
}

fn export_file_hint(export_file: &ExportFile) -> TokenStream {
    let filter = export_file
        .filter
        .as_ref()
        .map(|(_, lit)| quote::quote! { vec![#lit.into()]})
        .unwrap_or(quote::quote! {vec![]});
    quote::quote! {
        .with_hint(gdnative::nativescript::init::property::StringHint::File(
            gdnative::nativescript::init::property::EnumHint::new(#filter)))
    }
}

fn export_dir_hint() -> TokenStream {
    quote::quote! {
        .with_hint(gdnative::nativescript::init::property::StringHint::Dir)
    }
}

fn export_global_file_hint(export_global_file: &ExportGlobalFile) -> TokenStream {
    let filter = export_global_file
        .filter
        .as_ref()
        .map(|(_, lit)| quote::quote! { vec![#lit.into()]})
        .unwrap_or(quote::quote! {vec![]});
    quote::quote! {
        .with_hint(gdnative::nativescript::init::property::StringHint::GlobalFile(
            gdnative::nativescript::init::property::EnumHint::new(#filter)))
    }
}

fn export_global_dir_hint() -> TokenStream {
    quote::quote! {
        .with_hint(gdnative::nativescript::init::property::StringHint::GlobalDir)
    }
}

fn export_multiline_hint() -> TokenStream {
    quote::quote! {
        .with_hint(gdnative::nativescript::init::property::StringHint::Multiline)
    }
}

fn export_color_no_alpha_hint() -> TokenStream {
    quote::quote! {
        .with_hint(gdnative::nativescript::init::property::ColorHint::NoAlpha)
    }
}

fn is_number(ty: &Type) -> bool {
    is_int(ty) || is_float(ty)
}

fn is_int(ty: &Type) -> bool {
    ty == &parse_quote!(u8)
        || ty == &parse_quote!(u16)
        || ty == &parse_quote!(u32)
        || ty == &parse_quote!(u64)
        || ty == &parse_quote!(u128)
        || ty == &parse_quote!(usize)
        || ty == &parse_quote!(i8)
        || ty == &parse_quote!(i16)
        || ty == &parse_quote!(i32)
        || ty == &parse_quote!(i64)
        || ty == &parse_quote!(i128)
        || ty == &parse_quote!(isize)
}

fn is_float(ty: &Type) -> bool {
    ty == &parse_quote!(f32) || ty == &parse_quote!(f64)
}
