use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::quote;

use crate::tables::insert::insert_struct::create_insert_struct;
use crate::tables::table_data::TableData;

pub mod insert_struct;
pub mod impl_insert;

pub fn add_insert(table_data: &TableData) -> TokenStream {
    let fields = table_data
        .fields
        .iter()
        .filter(|f| !f.auto_increment)
        .collect_vec();

    let sql = format!(
        "INSERT INTO `{}` ({}) VALUES ({}) RETURNING *",
        table_data.struct_ident.to_string(),
        fields.iter().map(|f| format!("`{}`", f.db_name)).join(", "),
        fields
            .iter()
            .map(|f| format!("`{{self.{}}}`", f.rust_name()))
            .join(", ")
    );

    let struct_name = &table_data.struct_ident;
    let insert_struct = create_insert_struct(&table_data);

    quote! {
        impl #struct_name {
            /// Delete this row in the database
            pub async fn insert(self, conn: &mut sqlx::SqliteConnection) -> Result<Self, sqlx::Error> {
                let sql;
                sea_query::raw_sql!(
                    sqlx::sqlite::query_as,
                    sql = #sql
                ).fetch_one(conn).await
            }
        }

        #insert_struct
    }
}
