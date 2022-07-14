use std::collections::HashSet;

use darling::FromField;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

#[derive(Clone, Debug, FromField)]
#[darling(forward_attrs(cfg))]
pub struct StructField {
    pub ident: Option<syn::Ident>,
    pub ty: syn::Type,
}

pub fn generate_code(data: darling::ast::Data<(), StructField>) -> TokenStream2 {
    let fields = if let darling::ast::Data::Struct(ref struct_data) = data {
        Some(struct_data)
    } else {
        None
    };

    let unique_dependencies = fields
        .unwrap()
        .iter()
        .map(|e| e.ty.clone())
        .collect::<HashSet<_>>()
        .into_iter()
        .map(|e| fields.unwrap().iter().find(|f| f.ty == e).unwrap())
        .collect::<Vec<_>>();

    let dependencies_of_fields = unique_dependencies.iter().map(|e| {
        let field_name = &e.ident;
        let field_type = &e.ty;
        quote! {
            if infuse::service::is_service::<#field_type>() {
                const #field_name: [std::any::TypeId; #field_type::local_dependency_count] = #field_type::dependencies;

                Some((&#field_name, #field_type::local_dependency_count))
                } else {
                None
            }
        }
    }).collect::<Vec<_>>();

    let field_types = unique_dependencies
        .iter()
        .map(|el| el.ty.to_owned())
        .collect::<Vec<_>>();
    let dependency_count = field_types.len();

    // list + count unique local dependencies
    //

    let total_dependencies_count = if dependency_count > 0 {
        let sum_of_all = field_types.iter().map(|ty|{
            quote!{
                {if infuse::service::is_service::<#ty>() { #ty::local_dependency_count } else { 0 }}
            }
        }).collect::<Vec<_>>();

        quote! {
            infuse::dependencies::sum([#({#sum_of_all}, )*])
        }
        /* quote! {
            #({})*
        } */
    } else {
        quote! {
            1
        }
    };

    quote! {
        const dependencies: [std::any::TypeId; #dependency_count] = {

            const dependent_dependencies: [Option<(&'static [std::any::TypeId], usize)>; #dependency_count] = [#({#dependencies_of_fields}, )*];
            const self_dependent: [std::any::TypeId; #dependency_count] = [#(std::any::TypeId::of::<#field_types>() , )*];


            self_dependent
        };


        const local_dependency_count: usize = #dependency_count;
        const remote_dependency_count: usize = {#total_dependencies_count};
    }
}
