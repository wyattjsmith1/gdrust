use crate::compiler::hints::property_hint;
use crate::compiler::signal_arg::create_signal_arg;
use crate::parser::gdscript_class::GdScriptClass;
use crate::parser::gdscript_signal::GdScriptSignal;
use crate::parser::gdscript_var::{ExportType, GdScriptVar};
use proc_macro2::TokenStream;

pub fn gd_impl(class: &GdScriptClass) -> TokenStream {
    let class_name = &class.name;
    let ty = &class.parent();
    let init_vars = init_vars(class);
    let register_with = register_with_fn(class);
    quote::quote! {
        #[gdnative::methods]
        impl #class_name {
            fn new(_owner: &#ty) -> Self {
                Self {
                    #(#init_vars,)*
                }
            }
            #register_with
        }
    }
}

fn init_vars(class: &GdScriptClass) -> Vec<TokenStream> {
    class
        .vars()
        .iter()
        .map(|x| {
            let ident = &x.var_name;
            let default = &x.value;
            quote::quote! {
                #ident: #default
            }
        })
        .collect()
}

fn register_with_fn(class: &GdScriptClass) -> TokenStream {
    let properties: Vec<TokenStream> = class.vars().iter().map(|x| builder_for_var(x)).collect();
    let signals: Vec<TokenStream> = class
        .signals()
        .iter()
        .map(|x| builder_for_signal(x))
        .collect();
    quote::quote! {
        fn __register_properties_and_signals(builder: &gdnative::prelude::ClassBuilder<Self>) {
            #(#properties)*

            #(#signals)*
        }
    }
}

fn builder_for_var(var: &GdScriptVar) -> TokenStream {
    if let ExportType::NoExport(_) = var.export {
        return quote::quote! {};
    }
    let ty = &var.ty;
    let ident = &var.var_name;
    let default = &var.value;
    let ident_str = ident.to_string();
    let hint = property_hint(&var.export, &var.ty);
    let setter = quote::quote! { .with_setter(|this, _owner, val| {
        gdnative::godot_print!("Setting {:?} = {:?}", &#ident_str, &val);
        this.#ident = val
    })};
    let getter = quote::quote! { .with_ref_getter(|this, _owner| {
        gdnative::godot_print!("Getting {:?} = {:?}", &#ident_str, &this.#ident);
        &this.#ident
    })};
    let default = quote::quote! { .with_default(#default) };
    quote::quote! {
        builder.add_property::<#ty>(#ident_str)
            #hint
            #getter
            #setter
            #default
            .done();
    }
}

fn builder_for_signal(signal: &GdScriptSignal) -> TokenStream {
    let name_str = signal.name.to_string();
    let args: Vec<TokenStream> = signal
        .signal_type
        .iter()
        .map(|x| create_signal_arg(x))
        .collect();
    quote::quote! {
        builder.add_signal(gdnative::nativescript::Signal {
            name: #name_str,
            args: &[
                #(#args,)*
            ]
        });
    }
}
