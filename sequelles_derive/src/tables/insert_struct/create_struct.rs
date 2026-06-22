use itertools::Itertools;
use proc_macro2::Ident;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Field;

use crate::models::database_data::StructData;
use crate::tables::insert::impl_insert_trait;

pub fn create_insert_struct(data: &StructData) -> TokenStream {
    // Struct
    let struct_name = Ident::new(&format!("{}Insert", &data.struct_name), Span::call_site());
    let struct_doc = format!(
        "A version of [{}] that have default columns turned into options to let the database set the defaults itself.",
        data.struct_name
    );

    let struct_fields = data
        .struct_fields
        .iter()
        .map(|f| write_field(f, data))
        .collect_vec();

    quote! {
        #[doc = #struct_doc]
        #[derive(sequelles::bon::Builder)]
        pub struct #struct_name {
            #(#struct_fields),*
        }
    }
}

fn write_field(field: &Field, data: &StructData) -> TokenStream {
    let name = &field.ident;
    let typ = &field.ty;
    let table_field = data
        .table
        .fields
        .get_field_with_rust_name(&field.ident.as_ref().unwrap().to_string())
        .unwrap();

    if table_field.auto_increment || table_field.default {
        quote! {
            #[builder(into)]
            pub #name: Option<#typ>
        }
    } else {
        quote! {
            #[builder(into)]
            pub #name: #typ
        }
    }
}
