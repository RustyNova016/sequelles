use itertools::Itertools;
use proc_macro2::Ident;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Field;

use crate::models::database_data::StructData;

pub struct CreateInsertStruct;

impl CreateInsertStruct {
    pub fn get_struct_ident(data: &StructData) -> Ident {
        Ident::new(&format!("{}Insert", &data.struct_name), Span::call_site())
    }
}

pub fn create_insert_struct(data: &StructData) -> TokenStream {
    // Struct
    let struct_name = Ident::new(&format!("{}Insert", &data.struct_name), Span::call_site());
    let struct_doc = format!(
        "A version of [{}] that have default columns turned into [sequelles::InsertedValue] to let the database set the defaults itself.",
        data.struct_name
    );

    let struct_fields = data
        .struct_fields
        .iter()
        .map(|f| write_field(f, data))
        .collect_vec();

    let get_key_funcs = CreateInsertStruct::gen_get_key_funcs(data);
    let selsert = CreateInsertStruct::impl_selsert_trait(data);
    let from_row = CreateInsertStruct::impl_from_row_struct(
        &data.struct_name,
        &struct_name,
        &data.struct_fields,
    );

    quote! {
        #[doc = #struct_doc]
        #[derive(sequelles::bon::Builder)]
        pub struct #struct_name {
            #(#struct_fields),*
        }

        #get_key_funcs
        #selsert
        #from_row
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
