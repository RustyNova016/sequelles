use itertools::Itertools;
use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::quote;

use crate::tables::table_data::TableData;

pub fn create_selectable_impl(
    struct_name: &Ident,
    table_data: &TableData,
    uindex: &str,
) -> TokenStream {
    let sql = format!(
        "SELECT * FROM `{}` WHERE {}",
        table_data.struct_ident.to_string(),
        table_data
            .get_fields_with_uindex(uindex)
            .map(|f| f.as_sql_filter_binding())
            .join(" AND ")
    );

    let derive_struct = &table_data.struct_ident;

    quote! {
        impl sequelles::SelectUnique<&mut sqlx::SqliteConnection, #struct_name> for #derive_struct {
            async fn select_unique(
                conn: &mut sqlx::SqliteConnection,
                filter: #struct_name,
            ) -> Result<Option<Self>, sqlx::Error> {
                let filter: #struct_name = filter.into();
                let sql;
                sea_query::raw_sql!(
                    sqlx::sqlite::query_as,
                    sql = #sql
                ).fetch_optional(conn).await
            }
        }
    }
}
