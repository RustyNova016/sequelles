use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::quote;

use crate::tables::insert::impl_insert::impl_insert;
use crate::tables::table_data::TableData;
use crate::tables::table_data::field_data::FieldData;

pub fn create_insert_struct(table_data: &TableData) -> TokenStream {
    // Struct
    let derived_struct = &table_data.struct_ident;
    let struct_name = table_data.get_insert_struct_name();
    let struct_doc = format!(
        "A version of [{derived_struct}] that have default columns turned into options to let the database set the defaults itself."
    );

    let struct_fields = table_data.fields.iter().map(write_field).collect_vec();

    let insert1 = impl_insert(
        &struct_name,
        &quote! {&mut sqlx::SqliteConnection},
        &derived_struct,
        &table_data.db_name,
        &table_data.fields.iter().collect_vec(),
    );

    let insert2 = impl_insert(
        &derived_struct,
        &quote! {&mut sqlx::SqliteConnection},
        &derived_struct,
        &table_data.db_name,
        &table_data.fields.iter().collect_vec(),
    );

    quote! {
        #[doc = #struct_doc]
        #[derive(sequelles::bon::Builder)]
        pub struct #struct_name {
            #(#struct_fields),*
        }

        #insert1
        #insert2
    }
}

fn write_field(field: &FieldData) -> TokenStream {
    let name = &field.field.ident;
    let typ = &field.field.ty;
    if field.auto_increment || field.default {
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
