extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn name_timestamp(_attr: TokenStream, func: TokenStream) -> TokenStream {
    let func = parse_macro_input!(func as ItemFn);
    let func_vis = &func.vis;
    let func_block = &func.block;

    let func_decl = &func.sig;
    let func_name = &func_decl.ident;
    let func_generics = &func_decl.generics;
    let func_inputs = &func_decl.inputs;
    let func_output = &func_decl.output;

    let caller = quote! {
        #func_vis fn #func_name #func_generics(#func_inputs)#func_output{
            let current_time = chrono::Local::now();
            println!("==>> Func Start: {}() {}", stringify!(#func_name), current_time.format("%Y-%m-%d %H:%M:%S"));
            #func_block
            println!("==>> Func End: {}() {}", stringify!(#func_name), current_time.format("%Y-%m-%d %H:%M:%S"));
        }
    };
    caller.into()
}
