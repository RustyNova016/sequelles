use itertools::Itertools;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Field;
use syn::Ident;

use crate::models::database_data::StructData;

pub mod impl_postgres_insert;

pub struct CreateDynInsertableStruct;

impl CreateDynInsertableStruct {
    pub fn get_ident(row_ident: &Ident) -> Ident {
        Ident::new(&format!("{row_ident}InsertableDyn"), Span::call_site())
    }

    pub fn generate(data: &StructData) -> TokenStream {
        if !data.gen_dyn_insert_struct {
            return quote! {};
        }

        let struc = Self::declare_struct(data);
        let impl_insert = Self::create_impl_inserts(data);

        quote! {
            #struc

            #impl_insert
        }
    }

    fn declare_struct(data: &StructData) -> TokenStream {
        let struct_name = Self::get_ident(&data.struct_name);
        let struct_doc = format!(
            "A version of [{}] that allows for dynamically made inserts
            
            Please note that the SQL query is made **at runtime**, and may impact performance over an handwritten one.
            ",
            data.struct_name
        );

        let struct_fields = data
            .struct_fields
            .iter()
            .map(|f| create_struct_field(f, data))
            .collect_vec();

        quote! {
            #[doc = #struct_doc]
            #[derive(sequelles::bon::Builder)]
            pub struct #struct_name {
                #(#struct_fields),*
            }
        }
    }
}

fn create_struct_field(field: &Field, data: &StructData) -> TokenStream {
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
            pub #name: sequelles::InsertedValue<#typ>
        }
    } else {
        quote! {
            #[builder(into)]
            pub #name: #typ
        }
    }
}
