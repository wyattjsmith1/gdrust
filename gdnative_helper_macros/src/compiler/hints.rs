use crate::parser::gdscript_var::{ExportRange, ExportType};
use proc_macro2::TokenStream;
use syn::{parse_quote, Lit, Type};

pub(crate) fn property_hint(export: &ExportType, ty: &Type) -> TokenStream {
    match export {
        ExportType::NoHint | ExportType::Export(_) => quote::quote! {},
        ExportType::NoExport(_) => {
            panic!("Should only call property_hint if there is an export. Found NoExport")
        }
        ExportType::ExportRange(export_range) => export_range_hint(export_range, ty),
    }
}

fn export_range_hint(range: &ExportRange, ty: &Type) -> TokenStream {
    assert!(is_number(ty), "Export range must be a number (int, float)");
    assert!(
        range.range.len() >= 2,
        "Export range must contain at least a min and a max"
    );
    let min = &range.range[0];
    let max = &range.range[1];
    let mut step = TokenStream::default();
    let mut or_lesser = TokenStream::default();
    let mut or_greater = TokenStream::default();
    let mut current_index = 2;
    while current_index < range.range.len() {
        match &range.range[current_index] {
            Lit::Int(int) => step = quote::quote! { .with_step(#int)},
            Lit::Float(float) => step = quote::quote! { .with_step(#float)},
            Lit::Str(str) => match str.value().as_str() {
                "or_lesser" => or_lesser = quote::quote! { .or_lesser() },
                "or_greater" => or_greater = quote::quote! { .or_greater() },
                _ => panic!("Unexpected string literal. Expected \"or_lesser\" or \"or_greater\""),
            },
            _ => panic!(
                "Unexpected item in range: {:?}",
                &range.range[current_index]
            ),
        }
        current_index += 1;
    }
    if is_float(ty) {
        quote::quote! {
            .with_hint(gdnative::nativescript::init::property::FloatHint::Range(
                gdnative::nativescript::property::RangeHint::new(#min, #max)
                    #step
                    #or_lesser
                    #or_greater
            ))
        }
    } else {
        quote::quote! {
            .with_hint(gdnative::nativescript::init::property::IntHint::Range(
                gdnative::nativescript::property::RangeHint::new(#min, #max)
                    #step
                    #or_lesser
                    #or_greater
            ))
        }
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
