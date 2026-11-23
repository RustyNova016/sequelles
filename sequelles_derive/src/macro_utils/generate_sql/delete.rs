use core::fmt::Display;

use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::quote;

use crate::macro_utils::table_definition::unique_key::UniqueKeyData;

pub fn generate_delete_sql(table_name: impl Display, primary_key: &UniqueKeyData) -> TokenStream {
    let delete_where = primary_key
        .fields
        .iter()
        .map(|field| format!("`{}` = ?", field.db_name))
        .join(" AND ");

    let sql = format!("DELETE FROM `{table_name}` WHERE {delete_where}",);

    let binds = primary_key.fields.iter().map(|f| {
        let f = &f.field.ident;
        quote! {.bind(&self.#f)}
    });

    quote! {
        sequelles::sqlx::query(#sql)
            #(#binds)*
            .execute(conn).await?;
        Ok(())
    }
}
