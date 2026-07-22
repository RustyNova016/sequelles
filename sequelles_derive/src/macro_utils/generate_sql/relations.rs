use core::fmt::Display;

use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::quote;

use crate::macro_utils::table_definition::relation::RelationData;



// pub fn generate_one_to_many_sql(local_table: impl Display, foreign_key: &RelationData) -> TokenStream {
//     let foreign_table = &foreign_key.foreign_table;
    
//     let sql = format!(
//         "SELECT * FROM {foreign_table} WHERE",

//         fields.iter().map(|f| format!("`{}`", f.db_name)).join(", "),
//         fields.iter().map(|_| String::from("?")).join(", ")
//     );

//     let binds = fields.iter().map(|f| {
//         let f = &f.field.ident;
//         quote! {.bind(&self.#f)}
//     });

//     quote! {
//         sequelles::sqlx::query_as(#sql)
//             #(#binds)*
//             .fetch_optional(conn).await
//     }
// }
