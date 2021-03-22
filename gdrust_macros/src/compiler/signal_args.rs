use crate::compiler::signals::SignalArgDecl;
use proc_macro2::TokenStream;
use syn::parse_quote;

#[allow(clippy::module_name_repetitions)]
pub fn create_signal_arg(arg: &SignalArgDecl) -> TokenStream {
    let name_str = arg.name.to_string();
    let default = if let Some((_, default)) = arg.default.as_ref() {
        quote::quote! { gdnative::core_types::ToVariant::to_variant(&#default)}
    } else {
        quote::quote! { gdnative::core_types::Variant::new() }
    };
    let export_info = export_info(arg);
    quote::quote! {
        gdnative::nativescript::SignalArgument {
            name: #name_str,
            default: #default,
            export_info: #export_info,
            usage: gdnative::nativescript::PropertyUsage::DEFAULT,
        }
    }
}

fn export_info(arg: &SignalArgDecl) -> TokenStream {
    let ty = &arg.ty;
    if ty == &parse_quote! { Nil }
        || ty == &parse_quote! { Bool }
        || ty == &parse_quote! { I64 }
        || ty == &parse_quote! { F64 }
        || ty == &parse_quote! { GodotString }
        || ty == &parse_quote! { Vector2 }
        || ty == &parse_quote! { Rect2 }
        || ty == &parse_quote! { Vector3 }
        || ty == &parse_quote! { Transform2D }
        || ty == &parse_quote! { Plane }
        || ty == &parse_quote! { Quat }
        || ty == &parse_quote! { Aabb }
        || ty == &parse_quote! { Basis }
        || ty == &parse_quote! { Transform }
        || ty == &parse_quote! { Color }
        || ty == &parse_quote! { NodePath }
        || ty == &parse_quote! { Rid }
        || ty == &parse_quote! { Object }
        || ty == &parse_quote! { Dictionary }
        || ty == &parse_quote! { VariantArray }
        || ty == &parse_quote! { ByteArray }
        || ty == &parse_quote! { Int32Array }
        || ty == &parse_quote! { Float32Array }
        || ty == &parse_quote! { StringArray }
        || ty == &parse_quote! { Vector2Array }
        || ty == &parse_quote! { Vector3Array }
        || ty == &parse_quote! { ColorArray }
    {
        quote::quote! {
            gdnative::nativescript::ExportInfo::new(gdnative::core_types::VariantType::#ty)
        }
    } else {
        quote::quote! {
            gdnative::nativescript::ExportInfo::resource_type::<#ty>()
        }
    }
}
