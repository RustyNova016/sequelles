use core::fmt::Display;

use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::quote;

use crate::tables::table_data::field_data::FieldData;

pub fn generate_delete_sql(table_name: impl Display, pk_fields: &[&FieldData]) -> TokenStream {
    let delete_where = pk_fields
        .iter()
        .map(|field| format!("`{}` = ?", field.db_name))
        .join(" AND ");

    let sql = format!("DELETE FROM `{table_name}` WHERE {delete_where}",);

    let binds = pk_fields.iter().map(|f| {
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
