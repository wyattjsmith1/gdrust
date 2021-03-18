pub(crate) mod hints;
mod impl_block;

use crate::parser::gdscript_class::GdScriptClass;
use proc_macro2::TokenStream;

pub(crate) fn compile(parsed: &GdScriptClass) -> TokenStream {
    let gd_struct = create_struct(parsed);
    let gd_impl = impl_block::gd_impl(parsed);
    quote::quote! {
        #gd_struct
        #gd_impl
    }
}

fn create_struct(class: &GdScriptClass) -> TokenStream {
    let ident = &class.name;
    let vars: Vec<TokenStream> = class
        .vars()
        .iter()
        .map(|var| {
            let ident = &var.var_name;
            let ty = &var.ty;
            quote::quote! {
                #ident: #ty
            }
        })
        .collect();
    let parent = class.parent();
    quote::quote! {
        #[derive(gdnative::NativeClass)]
        #[inherit(#parent)]
        #[register_with(Self::__register_properties_and_signals)]
        pub struct #ident {
            #(#vars,)*
        }
    }
}
