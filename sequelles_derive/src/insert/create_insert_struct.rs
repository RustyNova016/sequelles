use core::fmt::Display;

use itertools::Itertools;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::insert::impl_insert_trait;
use crate::macro_utils::table_definition::field_data::FieldData;

pub fn create_insert_struct(
    derived_struct: &Ident,
    fields: &[&FieldData],
    table_name: impl Display,
) -> TokenStream {
    // Struct

    let struct_name = Ident::new(&format!("{}Insert", &derived_struct), Span::call_site());
    let struct_doc = format!(
        "A version of [{derived_struct}] that have default columns turned into options to let the database set the defaults itself."
    );

    let struct_fields = fields.iter().map(|f| write_field(f)).collect_vec();

    let insert = impl_insert_trait(
        &struct_name,
        &quote! {&mut sqlx::SqliteConnection},
        derived_struct,
        table_name,
        fields,
    );

    quote! {
        #[doc = #struct_doc]
        #[derive(sequelles::bon::Builder)]
        pub struct #struct_name {
            #(#struct_fields),*
        }

        #insert
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
