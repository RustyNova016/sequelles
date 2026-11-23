use core::fmt::Display;

use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::quote;

use crate::macro_utils::table_definition::field_data::FieldData;

pub fn generate_insert_sql(table_name: impl Display, table_fields: &[&FieldData]) -> TokenStream {
    let sql = format!(
        "INSERT INTO `{}` ({}) VALUES ({}) RETURNING *",
        table_name,
        table_fields
            .iter()
            .map(|f| format!("`{}`", f.db_name))
            .join(", "),
        table_fields.iter().map(|_| String::from("?")).join(", ")
    );

    let binds = table_fields.iter().map(|f| {
        let f = &f.field.ident;
        quote! {.bind(&self.#f)}
    });

    quote! {
        sequelles::sqlx::query_as(#sql)
            #(#binds)*
            .fetch_optional(conn).await
    }
}
