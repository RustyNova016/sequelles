use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::quote;

use crate::tables::table_data::TableData;

pub fn add_delete(table_data: &TableData) -> TokenStream {
    let sql = format!(
        "DELETE FROM `{}` WHERE {}",
        table_data.struct_ident.to_string(),
        table_data
            .get_fields_with_uindex("pk")
            .map(|f| f.as_sql_self_binding())
            .join(" AND ")
    );

    quote! {
        /// Delete this row in the database
        pub async fn delete(self, conn: &mut sqlx::SqliteConnection) -> Result<(), sqlx::Error> {
            let sql;
            sea_query::raw_sql!(
                sqlx::sqlite::query,
                sql = #sql
            ).execute(conn).await?;
            Ok(())
        }
    }
}
