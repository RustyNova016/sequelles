use core::fmt::Display;

use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::macro_utils::generate_sql::delete::generate_delete_sql;
use crate::tables::table_data::field_data::FieldData;

pub fn impl_delete_trait(
    for_type: &Ident,
    for_conn: &TokenStream,
    table_name: impl Display,
    pk_fields: &[&FieldData],
) -> TokenStream {
    let sql = generate_delete_sql(table_name, pk_fields);

    quote! {
        impl sequelles::Delete<#for_conn> for #for_type {
            async fn delete(self, conn: #for_conn) -> Result<(), sqlx::Error> {
                #sql
            }
        }
    }
}
