use proc_macro2::TokenStream;
use quote::quote;
use syn::Field;

use crate::tables::attributes::ColumnAtribute;
use crate::tables::delete::add_delete;
use crate::tables::insert::add_insert;
use crate::tables::relations::create_relations;
use crate::tables::select::add_select;
use crate::tables::select::filter_struct::create_filter_struct;
use crate::tables::table_data::TableData;
use crate::tables::uindexes::crate_all_unindexes;
use crate::tables::uindexes::create_uindex_struct;
use crate::tables::update::add_update;

pub mod attributes;
pub mod delete;
pub mod insert;
pub mod relations;
pub mod select;
pub mod table_data;
pub mod uindexes;
pub mod update;

pub fn impl_hello_macro(item: TokenStream) -> TokenStream {
    // Parse into an AST
    let mut input = syn::parse2::<syn::DeriveInput>(item).unwrap();

    // Parse the table data
    let table_data = TableData::parse(&mut input);

    // Build the macro

    let indexes = crate_all_unindexes(&table_data);

    let struct_name = &table_data.struct_ident;
    let pk_select = add_select(&table_data);
    let update = add_update(&table_data);
    let delete = add_delete(&table_data);
    let insert = add_insert(&table_data);

    let selects = create_filter_struct(&table_data);

    let relations = create_relations(&table_data);

    quote! {
        #indexes

        impl #struct_name {

            #pk_select
            #update
            #delete

        }

        #insert
        #selects
        #(#relations)*
    }
}
