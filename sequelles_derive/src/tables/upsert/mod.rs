use proc_macro2::TokenStream;
use quote::quote;

use sequelles_sql_gen::models::dialects::PostgreSQLDialect;
use sequelles_sql_gen::models::dialects::SqliteDialect;
use sequelles_sql_gen::models::queries::upsert::HasUpsert;

use crate::macro_utils::generate_binds::generate_self_binds;
use crate::models::database_data::StructData;
use crate::models::sql_dialects::SqlxConnectionType;
use crate::tables::error::get_error_type;
use crate::tables::error::get_snafu_type;

pub fn impl_upsert_trait(data: &StructData) -> TokenStream {
    if !data.gen_upsert {
        return quote! {};
    }

    let mut tokens = quote! {};

    if data.postgres {
        let imple = impl_upsert_trait_for_dialect::<PostgreSQLDialect>(&data);
        tokens = quote! {#tokens #imple};
    }

    if data.sqlite {
        let imple = impl_upsert_trait_for_dialect::<SqliteDialect>(&data);
        tokens = quote! {#tokens #imple};
    }

    tokens
}

fn impl_upsert_trait_for_dialect<L>(data: &StructData) -> TokenStream
where
    L: SqlxConnectionType + HasUpsert,
{
    let for_struct = &data.struct_name;

    let sql_statement = L::upsert(
        &data.table.get_table_identifier(),
        &data.table.fields,
        &data.table.iter_columns_not_in_pk().cloned().collect(),
        &data.table.unique_keys(),
    );

    let sql = sql_statement.sql;
    let binds = generate_self_binds(&sql_statement.binds);
    let for_conn = L::get_mut_connection();

    let context = get_snafu_type(data, "Upsert").map(|name| quote! {.context(#name)});
    let error = get_error_type(data);

    quote! {
        impl sequelles::Upsert<#for_conn> for #for_struct {
            type Output = Self;
            type Error = #error;

            async fn upsert(&self, conn: #for_conn) -> Result<Option<Self>, Self::Error> {
                use sequelles::snafu::ResultExt as _;
                
                sequelles::sqlx::query_as(#sql)
                    #binds
                    .fetch_optional(conn)
                    .await
                    #context
            }
        }
    }
}
