use core::fmt::Display;

use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::macro_utils::generate_sql::update::generate_update_sql;
use crate::tables::table_data::field_data::FieldData;

pub fn impl_update_trait(
    for_type: &Ident,
    for_conn: &TokenStream,
    table_name: impl Display,
    fields: &[&FieldData],
    pk_fields: &[&FieldData],
) -> TokenStream {
    let sql = generate_update_sql(table_name, fields, pk_fields);

    quote! {
        impl sequelles::Update<#for_conn> for #for_type {
            /// Update the row in the database.
            ///
            /// This does a `UPDATE ... RETURNING *;`, which means the current struct will be updated with the actual data in the database (Which may be modified by triggers).
            async fn update(&mut self, conn: #for_conn) -> Result<(), sqlx::Error> {
                #sql
            }
        }
    }
}
