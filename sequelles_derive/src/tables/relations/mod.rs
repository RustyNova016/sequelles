use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::quote;

use crate::tables::table_data::TableData;
use crate::tables::table_data::field_data::FieldData;

pub fn create_relations(table_data: &TableData) -> Vec<TokenStream> {
    table_data
        .fields
        .iter()
        .filter_map(|f| create_relation(f, &table_data.db_name))
        .collect_vec()
}

pub fn create_relation(field: &FieldData, db_table: &str) -> Option<TokenStream> {
    let relation_name = field.relation.as_ref()?.get_relation_struct_name(db_table);

    Some(quote! {
        pub struct #relation_name;
    })
}
