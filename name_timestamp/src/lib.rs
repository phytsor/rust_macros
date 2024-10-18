extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Name_Timestamp)]
pub fn name_timestamp_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_name_timestamp_macro(&ast)
}

fn impl_name_timestamp_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Name_Timestamp for #name{
            fn name_timestamp(){
                println!("");
            }
        }
    };
    gen.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
