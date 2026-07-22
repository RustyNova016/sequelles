use core::fmt::Display;
use core::marker::PhantomData;

use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::macro_utils::query_gen::select_key::SelectKeyQuery;
use crate::macro_utils::table_definition::key::TableKey;
use crate::macro_utils::table_definition::sql_dialect::Sqlite;

#[derive(Default)]
pub struct SelectOneImpl<D> {
    dialect: PhantomData<D>,
}

impl SelectOneImpl<Sqlite> {
    pub fn generate(
        table_name: impl Display,
        table_key: &TableKey,
        for_struct: &Ident,
        with_filter: &Ident,
    ) -> TokenStream {
        let sql = SelectKeyQuery::<Sqlite>::generate(table_name, table_key);
        let for_conn = Sqlite::get_conn();

        quote! {
            impl sequelles::SelectOne<#for_conn, #with_filter> for #for_struct {
                async fn select_one(conn: #for_conn, filter: #with_filter) -> Result<Option<Self>, sqlx::Error> {
                    #sql
                }
            }
        }
    }
}
