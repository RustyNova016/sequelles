pub mod create_filter_struct;
pub mod create_ukey_struct;
pub mod impl_select_key_trait;
pub mod ukey_struct_from;
use itertools::Itertools as _;
use proc_macro2::TokenStream;
use quote::quote;

use crate::select::create_filter_struct::create_filter_struct;
use crate::select::create_ukey_struct::create_ukey_struct;
use crate::tables::table_data::TableData;

pub fn impl_select_macro(item: TokenStream) -> TokenStream {
    // Parse into an AST
    let mut input = syn::parse2::<syn::DeriveInput>(item).unwrap();

    // Parse the table data
    let table_data = TableData::parse(&mut input);

    // Build the macro
    select_macro_inner(&table_data)
}

pub fn select_macro_inner(table_data: &TableData) -> TokenStream {
    let ukeys = table_data
        .get_uindex_names()
        .map(|ukey| {
            create_ukey_struct(
                &table_data.struct_ident,
                &table_data.db_name,
                ukey,
                &table_data.get_fields_with_uindex(ukey).collect_vec(),
            )
        })
        .collect_vec();

    let filter_struct = create_filter_struct(
        &table_data.struct_ident,
        &table_data.db_name,
        &table_data.fields.iter().collect_vec(),
    );

    quote! {
        #(#ukeys)*
        #filter_struct
    }
}
