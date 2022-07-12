extern crate darling;
extern crate syn;
use std::any::Any;

use proc_macro::TokenStream;
use quote::{quote};
use syn::{parse_macro_input, DeriveInput};
use url::Url;
use validator::validate_url;
use darling::{FromDeriveInput, FromField};

use crate::dependencies::{StructField, self};

#[derive(FromDeriveInput, Debug)]
#[darling(attributes(controller), forward_attrs(allow, doc, cfg))]
#[darling(supports(struct_named))]
struct ControllerOpts {
    ident: syn::Ident,
    path: Option<String>,
    data: darling::ast::Data<(), StructField>,
}

pub fn derive_controller(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match ControllerOpts::from_derive_input(&input) {
        Err(err) => err.write_errors().into(),
        Ok(data) => {
            println!("{:?}", data.data);
            let ident = &data.ident;
            let path = data
                .path
                .map(|e| {
                    if e.starts_with("/") {
                        e
                    } else {
                        format!("/{}", e)
                    }
                })
                .unwrap_or("/".to_string());

            if !validate_url(format!("http://0.0.0.0:8080{}", path)) {
                return syn::Error::new(data.ident.span(), format!("{} is not a valid path", path))
                    .into_compile_error()
                    .into();
            }

            let dummy_url: Url = format!("http://0.0.0.0:8080{}", path).parse().unwrap();

            let path_segments = dummy_url.path_segments().unwrap().collect::<Vec<_>>();
            let dynamic_path_segments = path_segments.iter().filter(|e|e.starts_with(":")).collect::<Vec<_>>();
            let path_segments_count = dynamic_path_segments.len();

            let dependency_code = dependencies::generate_code(data.data);

            quote! {
                impl #ident {
                    const name: &'static str = #path;
                    const dynamic_segments: [&'static str; #path_segments_count] = [#(#dynamic_path_segments, )*];
                    
                    #dependency_code
                    
                    pub fn new() {

                    }
                }
            }
            .into()
        }
    }
}