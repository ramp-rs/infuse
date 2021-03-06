#![feature(const_type_id)]

use proc_macro::TokenStream;
mod controller;
mod dependencies;
mod service;

#[proc_macro_derive(Controller, attributes(controller, get))]
pub fn derive_controller(input: TokenStream) -> TokenStream {
    controller::derive_controller(input)
}

#[proc_macro_derive(Service, attributes(service))]
pub fn derive_service(input: TokenStream) -> TokenStream {
    service::derive_service(input)
}
