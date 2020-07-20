extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct  a representation of Rust code as  syntax tree
    // proc_macro_derive functions must return TokenStream rather than Result to conform to the procedural macro API
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    // Build the trait implenenation
    impl_hello_macro(ast)
}

fn impl_hello_macro(ast: syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    // Build the output, possibly using quasi-quotation
    let gen = quote!{
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };

    // Convert into a token stream and return it
    gen.into()
}

