use core::fmt::Display;

use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::macro_utils::generate_sql::insert::generate_insert_sql;

// pub(super) fn impl_one_to_many(
//     relation_struct: &Ident,
//     for_conn: &TokenStream,

//     one_struct: &Ident,
//     one_table: &Ident,

//     many_struct: &Ident,
//     many_table: &Ident,

//     table_name: impl Display,
//     fields: &[&FieldData],
// ) -> TokenStream {
//     let insert = generate_insert_sql(table_name, fields);

//     quote! {
//         impl sequelles::OneToMany<#for_conn> for #for_type {
//             type Output = #output;

//             #[must_use]
//             async fn insert(&self, conn: #for_conn) -> Result<Option<Self::Output>, sqlx::Error> {
//                 #insert
//             }
//         }
//     }
// }
