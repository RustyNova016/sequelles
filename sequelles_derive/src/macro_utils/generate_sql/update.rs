use core::fmt::Display;

use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::quote;

use crate::tables::table_data::field_data::FieldData;

pub fn generate_update_sql(
    table_name: impl Display,
    fields: &[&FieldData],
    pk_fields: &[&FieldData],
) -> TokenStream {
    let update_sets = fields
        .iter()
        .map(|field| format!("`{}` = ?", field.db_name))
        .join(", ");

    let update_where = pk_fields
        .iter()
        .map(|field| format!("`{}` = ?", field.db_name))
        .join(" AND ");

    let sql = format!("UPDATE `{table_name}` SET {update_sets} WHERE {update_where} RETURNING *",);

    let binds = fields.iter().chain(pk_fields.iter()).map(|f| {
        let f = &f.field.ident;
        quote! {.bind(&self.#f)}
    });

    quote! {
        *self = sequelles::sqlx::query_as(#sql)
            #(#binds)*
            .fetch_one(conn).await?;
        Ok(())
    }
}
