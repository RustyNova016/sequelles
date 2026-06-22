use core::fmt::Display;

use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::macro_utils::generate_sql::select::generate_select_key_sql;
use crate::macro_utils::table_definition::unique_key::UniqueKeyData;

pub fn impl_select_trait(
    for_type: &Ident,
    for_conn: &TokenStream,
    with_filter: &Ident,
    table_name: impl Display,
    unique_key: &UniqueKeyData,
) -> TokenStream {
    let sql = generate_select_key_sql(table_name, unique_key);

    quote! {
        impl sequelles::SelectKey<#for_conn, #with_filter> for #for_type {
            async fn select_by_key(conn: #for_conn, filter: #with_filter) -> Result<Option<Self>, sqlx::Error> {
                #sql
            }
        }
    }
}
