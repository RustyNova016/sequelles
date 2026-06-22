use core::fmt::Display;
use std::rc::Rc;

use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::quote;

use crate::macro_utils::table_definition::field_data::FieldData;
use crate::macro_utils::table_definition::unique_key::UniqueKeyData;

pub fn generate_update_sql(
    table_name: impl Display,
    fields: &[&FieldData],
    primary_key: &UniqueKeyData,
) -> TokenStream {
    let update_sets = fields
        .iter()
        .map(|field| format!("`{}` = ?", field.db_name))
        .join(", ");

    let update_where = primary_key
        .fields
        .iter()
        .map(|field| format!("`{}` = ?", field.db_name))
        .join(" AND ");

    let sql = format!("UPDATE `{table_name}` SET {update_sets} WHERE {update_where} RETURNING *",);

    let binds = fields
        .into_iter()
        .map(|f| *f)
        .chain(primary_key.fields_as_ref_iter())
        .map(|f| {
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
