#![feature(proc_macro_quote)]

extern crate quote;
extern crate syn;
extern crate proc_macro2;
use proc_macro::TokenStream;
use syn::DeriveInput;
use quote::quote;

#[proc_macro_derive(CurvePoint)]
pub fn derive_curvepoint(input: TokenStream) -> TokenStream {
    let input = proc_macro2::TokenStream::from(input);
    let ast: DeriveInput = syn::parse2(input).unwrap();
    let _group_type = ast.generics;
    let output = match &ast.data {
        syn::Data::Struct(_s) => {
            quote!()
        },
        _ => {
            quote!()
        }
    };
    return proc_macro::TokenStream::from(output);
}
