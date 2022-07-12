use darling::FromField;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote};

#[derive(Clone, Debug, FromField)]
#[darling(forward_attrs(cfg))]
pub struct StructField {
    pub ident: Option<syn::Ident>,
    pub ty: syn::Type,
    vis: syn::Visibility,
    // If the line would be commented, all will be fine
    attrs: Vec<syn::Attribute>,
}

pub fn generate_code(data: darling::ast::Data<(), StructField>) -> TokenStream2 {
    let fields = if let darling::ast::Data::Struct(ref struct_data) = data {
        Some(struct_data)
    } else {
        None
    };

    let dependencies_of_fields = fields.unwrap().iter().map(|e| {
        let field_name = &e.ident;
        let field_type = &e.ty;
        quote! {
            const #field_name: &'static [std::any::TypeId] = #field_type::dependencies;

            #field_name
        }
    }).collect::<Vec<_>>();

    let field_types = fields.unwrap().iter().map(|el|el.ty.to_owned()).collect::<Vec<_>>();
    let dependency_count = field_types.len();

    quote! {
        const dependency_count: usize = #dependency_count;
        const dependencies: &'static [std::any::TypeId] = {
            const dependent_dependencies: [&'static [std::any::TypeId]; #dependency_count] = [#({#dependencies_of_fields}, )*];
            const self_dependent: [std::any::TypeId; #dependency_count] = [#(std::any::TypeId::of::<#field_types>(), )*];
            
            self_dependent
        };
    }
}