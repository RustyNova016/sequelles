use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::quote;

use crate::tables::table_data::TableData;

pub mod filter_struct;

pub fn add_select(table_data: &TableData) -> TokenStream {
    let fields = table_data
        .get_fields_with_uindex("pk")
        .map(|f| f.as_function_arg());

    let sql = format!(
        "SELECT * FROM `{}` WHERE {}",
        table_data.struct_ident.to_string(),
        table_data.get_fields_with_uindex("pk").map(|f| f.as_sql_binding()).join(" AND ")
    );

    quote! {
        /// Fetch a row by it's id.
        pub async fn select_by_pk(conn: &mut sqlx::SqliteConnection, #(#fields),*) -> Result<Option<Self>, sqlx::Error> {
            let sql;
            sea_query::raw_sql!(
                sqlx::sqlite::query_as,
                sql = #sql
            ).fetch_optional(conn).await
        }
    }
}
