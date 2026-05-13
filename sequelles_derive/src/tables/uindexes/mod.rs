use core::fmt::Display;

use convert_case::Case;
use convert_case::Casing;
use itertools::Itertools;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::tables::table_data::TableData;
use crate::tables::uindexes::from_value::create_from;
use crate::tables::uindexes::selects::create_selectable_impl;

pub mod from_value;
pub mod selects;

pub fn crate_all_unindexes(table: &TableData) -> TokenStream {
    let mut indexes = Vec::new();

    for ind in table.get_uindex_names() {
        indexes.push(create_uindex_struct(table, ind));
    }

    quote! {#(#indexes)*}
}

pub fn create_uindex_struct(table_data: &TableData, uindex: &str) -> TokenStream {
    let fields = table_data.get_fields_with_uindex(uindex).collect_vec();

    let struct_name = Ident::new(
        &format!(
            "{}{}",
            table_data.struct_ident.to_string(),
            uindex.to_case(Case::Pascal)
        ),
        Span::call_site(),
    );

    let from = create_from(&fields, &struct_name);
    let select_unique = create_selectable_impl(&struct_name, table_data, uindex);

    quote! {
        pub struct #struct_name {
            #(#fields),*
        }

        #from
        #select_unique
    }
}
