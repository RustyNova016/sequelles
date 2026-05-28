pub mod impl_delete_trait;
use itertools::Itertools as _;
use proc_macro2::TokenStream;
use quote::quote;

use crate::delete::impl_delete_trait::impl_delete_trait;
use crate::tables::table_data::TableData;

pub fn impl_delete_macro(item: TokenStream) -> TokenStream {
    // Parse into an AST
    let mut input = syn::parse2::<syn::DeriveInput>(item).unwrap();

    // Parse the table data
    let table_data = TableData::parse(&mut input);

    // Build the macro
    delete_macro_inner(&table_data)
}

pub fn delete_macro_inner(table_data: &TableData) -> TokenStream {
    let table_name = &table_data.db_name;
    let for_type = &table_data.struct_ident;
    let for_conn = &quote! {&mut sqlx::SqliteConnection};

    let pk_fields = &table_data.get_fields_with_uindex("pk").collect_vec();

    impl_delete_trait(for_type, for_conn, table_name, pk_fields)
}
