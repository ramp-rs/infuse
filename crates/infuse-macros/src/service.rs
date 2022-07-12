extern crate darling;
extern crate syn;
use proc_macro::TokenStream;
use quote::{quote};
use syn::{parse_macro_input, DeriveInput};
use url::Url;
use validator::validate_url;
use darling::FromDeriveInput;

use crate::dependencies::{StructField, self};

#[derive(FromDeriveInput, Debug)]
#[darling(attributes(service), forward_attrs(allow, doc, cfg))]
#[darling(supports(struct_named))]
struct ServiceOpts {
    ident: syn::Ident,
    path: Option<String>,
    data: darling::ast::Data<(), StructField>,
}

pub fn derive_service(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match ServiceOpts::from_derive_input(&input) {
        Err(err) => err.write_errors().into(),
        Ok(data) => {
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