use itertools::Itertools;
use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::quote;

use crate::macro_utils::table_definition::field_data::FieldData;

pub fn ukey_from(fields: &[&FieldData], struct_name: &Ident) -> TokenStream {
    if fields.is_empty() {
        quote! {}
    } else if fields.len() == 1 {
        ukey_from_value(fields.first().unwrap(), struct_name)
    } else {
        ukey_from_tupple(fields, struct_name)
    }
}

fn ukey_from_value(field: &FieldData, struct_name: &Ident) -> TokenStream {
    let field = &field.field;
    let typ = &field.ty;
    let ident = &field.ident;

    quote! {
        impl From<#typ> for #struct_name {
            fn from(value: #typ) -> Self {
                Self {
                    #ident: value
                }
            }
        }
    }
}

fn ukey_from_tupple(fields: &[&FieldData], struct_name: &Ident) -> TokenStream {
    let fields_tuple = fields
        .iter()
        .map(|f| {
            let typ = &f.field.ty;
            quote! {#typ}
        })
        .collect_vec();

    let type_fill = fields
        .iter()
        .enumerate()
        .map(|(num, field)| {
            let ident = &field.field.ident;
            let num = syn::Index::from(num);
            quote! {#ident: value.#num}
        })
        .collect_vec();

    quote! {
        impl From<(#(#fields_tuple),*)> for #struct_name {
            fn from(value: (#(#fields_tuple),*)) -> Self {
                Self {
                    #(#type_fill),*
                }
            }
        }
    }
}
