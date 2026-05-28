pub mod create_insert_struct;
pub mod impl_insert_trait;
use itertools::Itertools as _;
use proc_macro2::TokenStream;
use quote::quote;

use crate::insert::create_insert_struct::create_insert_struct;
use crate::insert::impl_insert_trait::impl_insert_trait;
use crate::tables::table_data::TableData;

pub fn impl_insert_macro(item: TokenStream) -> TokenStream {
    // Parse into an AST
    let mut input = syn::parse2::<syn::DeriveInput>(item).unwrap();

    // Parse the table data
    let table_data = TableData::parse(&mut input);

    // Build the macro
    insert_macro_inner(&table_data)
}

pub fn insert_macro_inner(table_data: &TableData) -> TokenStream {
    let table_name = &table_data.db_name;
    let derived_struct = &table_data.struct_ident;
    let for_conn = &quote! {&mut sqlx::SqliteConnection};

    let fields = &table_data.fields.iter().collect_vec();

    let impl_trait =
        impl_insert_trait(derived_struct, for_conn, derived_struct, table_name, fields);
    let insert_struct = create_insert_struct(derived_struct, fields, table_name);

    quote! {
        #impl_trait
        #insert_struct
    }
}
