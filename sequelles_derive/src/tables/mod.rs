pub mod dyn_insert_struct;
use proc_macro2::TokenStream;
use quote::quote;

use crate::models::database_data::StructData;
use crate::tables::delete::impl_delete_trait;
use crate::tables::dyn_insert_struct::CreateDynInsertableStruct;
use crate::tables::error::create_error_enum;
use crate::tables::insert::impl_insert_trait;
use crate::tables::insert_struct::impl_insert_structs;
use crate::tables::select::impl_select_trait;
use crate::tables::selsert::impl_selsert_trait;
use crate::tables::unique_key::impl_unique_structs;
use crate::tables::update::impl_update_trait;
use crate::tables::upsert::impl_upsert_trait;

pub mod delete;
pub mod error;
pub mod insert;
pub mod insert_struct;
pub mod select;
pub mod selsert;
pub mod unique_key;
pub mod update;
pub mod upsert;

pub fn impl_table_macro(item: TokenStream) -> TokenStream {
    // Parse into an AST
    let mut input = syn::parse2::<syn::DeriveInput>(item).unwrap();

    // Parse the table data
    let table_data = StructData::parse_struct(&mut input);

    // Build the macro
    //let relations = create_relations(&table_data);
    let delete = impl_delete_trait(&table_data);
    let update = impl_update_trait(&table_data);
    let insert = impl_insert_trait(&table_data);
    let select = impl_select_trait(&table_data);

    let insert_struct = impl_insert_structs(&table_data);
    let dyn_insert_struct = CreateDynInsertableStruct::generate(&table_data);
    let unique_struct = impl_unique_structs(&table_data);
    let upsert = impl_upsert_trait(&table_data);
    let selsert = impl_selsert_trait(&table_data, &table_data.struct_name, &quote! {Self});

    let snafu_impl = create_error_enum(&table_data);

    quote! {
        #snafu_impl
        #delete
        #update
        #insert
        #select
        #insert_struct
        #dyn_insert_struct
        #unique_struct
        #upsert
        #selsert
    }
}
