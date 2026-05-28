pub mod impl_update_trait;
use itertools::Itertools as _;
use proc_macro2::TokenStream;
use quote::quote;

use crate::tables::table_data::TableData;
use crate::update::impl_update_trait::impl_update_trait;

pub fn impl_update_macro(item: TokenStream) -> TokenStream {
    // Parse into an AST
    let mut input = syn::parse2::<syn::DeriveInput>(item).unwrap();

    // Parse the table data
    let table_data = TableData::parse(&mut input);

    // Build the macro
    update_macro_inner(&table_data)
}

pub fn update_macro_inner(table_data: &TableData) -> TokenStream {
    let table_name = &table_data.db_name;
    let for_type = &table_data.struct_ident;
    let for_conn = &quote! {&mut sqlx::SqliteConnection};

    let fields = &table_data.fields.iter().collect_vec();
    let pk_fields = &table_data.get_fields_with_uindex("pk").collect_vec();

    impl_update_trait(for_type, for_conn, table_name, fields, pk_fields)
}
