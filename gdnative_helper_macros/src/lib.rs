use proc_macro::TokenStream;
mod compiler;
mod parser;
use crate::parser::gdscript_class::GdScriptClass;

#[proc_macro]
pub fn gdrust(input: TokenStream) -> TokenStream {
    let parsed = syn::parse_macro_input!(input as GdScriptClass);
    let compiled = compiler::compile(&parsed);
    // println!("{}", compiled.to_string()); // Uncomment this to print the compiled code
    compiled.into()
}
