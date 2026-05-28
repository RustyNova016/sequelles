use core::fmt::Display;

use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::quote;

use crate::tables::table_data::field_data::FieldData;

pub fn generate_select_sql(table_name: impl Display, unique_fields: &[&FieldData]) -> TokenStream {
    let select_where = unique_fields
        .iter()
        .map(|field| format!("`{}` = ?", field.db_name))
        .join(" AND ");

    let sql = format!("SELECT * FROM `{table_name}` WHERE {select_where}",);

    let binds = unique_fields.iter().map(|f| {
        let f = &f.field.ident;
        quote! {.bind(&filter.#f)}
    });

    quote! {
        sequelles::sqlx::query_as(#sql)
            #(#binds)*
            .fetch_optional(conn).await
    }
}
