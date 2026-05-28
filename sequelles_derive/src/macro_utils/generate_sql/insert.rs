use core::fmt::Display;

use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::quote;

use crate::tables::table_data::field_data::FieldData;

pub fn generate_insert_sql(table_name: impl Display, fields: &[&FieldData]) -> TokenStream {
    let sql = format!(
        "INSERT INTO `{}` ({}) VALUES ({}) RETURNING *",
        table_name,
        fields.iter().map(|f| format!("`{}`", f.db_name)).join(", "),
        fields
            .iter()
            .map(|_| String::from("?"))
            .join(", ")
    );

    let binds =  fields.iter().map(|f| {
        let f = &f.field.ident;
        quote! {.bind(&self.#f)}
    });

    quote! {
        sequelles::sqlx::query_as(#sql)
            #(#binds)*
            .fetch_optional(conn).await
    }
}
