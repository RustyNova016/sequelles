use core::fmt::Display;

use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::macro_utils::generate_sql::insert::generate_insert_sql;
use crate::tables::table_data::field_data::FieldData;

pub(super) fn impl_insert_trait(
    for_type: &Ident,
    for_conn: &TokenStream,
    output: &Ident,
    table_name: impl Display,
    fields: &[&FieldData],
) -> TokenStream {
    let insert = generate_insert_sql(table_name, fields);

    quote! {
        impl sequelles::Insert<#for_conn> for #for_type {
            type Output = #output;

            #[must_use]
            async fn insert(&self, conn: #for_conn) -> Result<Option<Self::Output>, sqlx::Error> {
                #insert
            }
        }
    }
}
