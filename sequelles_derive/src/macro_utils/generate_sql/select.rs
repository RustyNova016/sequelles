use core::fmt::Display;

use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::quote;

use crate::macro_utils::table_definition::key::TableKey;
use crate::macro_utils::table_definition::unique_key::UniqueKeyData;

pub fn generate_select_key_sql(table_name: impl Display, table_key: &TableKey) -> TokenStream {
    let select_where = table_key
        .fields
        .iter()
        .map(|field| format!("`{}` = ?", field.db_name))
        .join(" AND ");

    let sql = format!("SELECT * FROM `{table_name}` WHERE {select_where}",);

    let binds = table_key.fields.iter().map(|f| {
        let f = &f.field.ident;
        quote! {.bind(&filter.#f)}
    });

    let fetch = if table_key.unique {
        quote! {fetch_optional}
    } else {
        quote! {fetch_all}
    };

    quote! {
        sequelles::sqlx::query_as(#sql)
            #(#binds)*
            .#fetch(conn).await
    }
}
