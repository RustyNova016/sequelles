use core::fmt::Display;

use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::tables::table_data::field_data::FieldData;

pub(super) fn impl_insert(
    for_type: &Ident,
    for_conn: &TokenStream,
    output: &Ident,
    table_name: impl Display,
    fields: &[&FieldData],
) -> TokenStream {
    let sql = format!(
        "INSERT INTO `{}` ({}) VALUES ({}) RETURNING *",
        table_name,
        fields.iter().map(|f| format!("`{}`", f.db_name)).join(", "),
        fields
            .iter()
            .map(|f| format!("{{self.{}}}", f.rust_name()))
            .join(", ")
    );

    quote! {
        impl sequelles::Insert<#for_conn> for #for_type {
            type Output = #output;

            async fn insert(&self, conn: #for_conn) -> Result<Self::Output, sqlx::Error> {
                let sql;
                sea_query::raw_sql!(
                    sqlx::sqlite::query_as,
                    sql = #sql
                ).fetch_one(conn).await
            }
        }
    }
}
