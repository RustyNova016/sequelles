use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::quote;

use crate::tables::table_data::TableData;

pub fn add_update(table_data: &TableData) -> TokenStream {
    let update_sets = table_data
        .fields
        .iter()
        .map(|field| format!("`{}` = {{self.{}}}", field.db_name, field.rust_name()))
        .join(", ");

    let update_where = table_data
        .get_fields_with_uindex("pk")
        .map(|field| format!("`{}` = {{self.{}}}", field.db_name, field.rust_name()))
        .join(" AND ");

    let sql = format!(
        "UPDATE `{}` SET {update_sets} WHERE {}",
        table_data.struct_ident.to_string(),
        update_where
    );

    quote! {
        /// Update the current row in the DB
        pub async fn update(&mut self, conn: &mut sqlx::SqliteConnection) -> Result<(), sqlx::Error> {
            let sql;
            sea_query::raw_sql!(
                sqlx::sqlite::query,
                sql = #sql
            ).execute(conn).await?;
            Ok(())
        }
    }
}
