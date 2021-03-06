extern crate darling;
extern crate syn;
use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

use crate::dependencies::{self, StructField};

#[derive(FromDeriveInput, Debug)]
#[darling(attributes(service), forward_attrs(allow, doc, cfg))]
#[darling(supports(struct_named))]
struct ServiceOpts {
    ident: syn::Ident,
    data: darling::ast::Data<(), StructField>,
}

pub fn derive_service(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match ServiceOpts::from_derive_input(&input) {
        Err(err) => err.write_errors().into(),
        Ok(data) => {
            println!("{:?}", data);
            let ident = data.ident;
            let dependency_code = dependencies::generate_code(data.data);
            quote! {

                impl infuse::service::Service for #ident{}


                impl #ident {
                    #dependency_code
                }
            }
            .into()
        }
    }
}
