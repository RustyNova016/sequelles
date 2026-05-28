use proc_macro2::TokenStream;
use quote::quote;

use crate::delete::delete_macro_inner;
use crate::insert::insert_macro_inner;
use crate::select::select_macro_inner;
use crate::tables::relations::create_relations;
use crate::tables::select::add_select;
use crate::tables::table_data::TableData;
use crate::update::update_macro_inner;

pub mod attributes;
pub mod relations;
pub mod select;
pub mod table_data;

pub fn impl_table_macro(item: TokenStream) -> TokenStream {
    // Parse into an AST
    let mut input = syn::parse2::<syn::DeriveInput>(item).unwrap();

    // Parse the table data
    let table_data = TableData::parse(&mut input);

    // Build the macro



    let struct_name = &table_data.struct_ident;
    let pk_select = add_select(&table_data);



    let relations = create_relations(&table_data);

    let insert = insert_macro_inner(&table_data);
    let select = select_macro_inner(&table_data);
    let update = update_macro_inner(&table_data);
    let delete = delete_macro_inner(&table_data);

    quote! {
        #insert
        #select
        #update
        #delete



        impl #struct_name {
            #pk_select

        }

        #(#relations)*
    }
}
