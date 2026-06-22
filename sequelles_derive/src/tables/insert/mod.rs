use proc_macro2::TokenStream;
use quote::quote;

use sequelles_sql_gen::models::dialects::PostgreSQLDialect;
use sequelles_sql_gen::models::dialects::SqliteDialect;
use sequelles_sql_gen::models::queries::insert::HasInsert;

use crate::macro_utils::generate_binds::generate_self_binds;
use crate::models::database_data::StructData;
use crate::models::sql_dialects::SqlxConnectionType;

pub fn impl_insert_trait(data: &StructData) -> TokenStream {
    if !data.gen_insert {
        return quote! {};
    }

    let mut tokens = quote! {};

    if data.postgres {
        let imple = impl_insert_trait_for_dialect::<PostgreSQLDialect>(&data);
        tokens = quote! {#tokens #imple};
    }

    if data.sqlite {
        let imple = impl_insert_trait_for_dialect::<SqliteDialect>(&data);
        tokens = quote! {#tokens #imple};
    }

    tokens
}

fn impl_insert_trait_for_dialect<L>(data: &StructData) -> TokenStream
where
    L: SqlxConnectionType + HasInsert,
{
    let for_struct = &data.struct_name;

    let sql_statement = L::insert(&data.table.get_table_identifier(), &data.table.fields, true);

    let sql = sql_statement.sql;
    let binds = generate_self_binds(&sql_statement.binds);
    let for_conn = L::get_mut_connection();

    quote! {
        impl sequelles::InsertOrIgnore<#for_conn> for #for_struct {
            type Output = Self;

            async fn insert_or_ignore(&self, conn: #for_conn) -> Result<Option<Self>, sqlx::Error> {
                sequelles::sqlx::query_as(#sql)
                    #binds
                    .fetch_optional(conn).await
            }
        }
    }
}
