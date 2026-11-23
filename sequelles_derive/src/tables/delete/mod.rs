use proc_macro2::TokenStream;
use quote::quote;

use sequelles_sql_gen::models::dialects::PostgreSQLDialect;
use sequelles_sql_gen::models::dialects::SqliteDialect;
use sequelles_sql_gen::models::queries::delete::HasDelete;

use crate::macro_utils::generate_binds::generate_self_binds;
use crate::models::database_data::StructData;
use crate::models::sql_dialects::SqlxConnectionType;
use crate::tables::error::get_error_type;
use crate::tables::error::get_snafu_type;

pub fn impl_delete_trait(data: &StructData) -> TokenStream {
    if !data.gen_delete {
        return quote! {};
    }

    let mut tokens = quote! {};

    if data.postgres {
        let imple = impl_delete_trait_for_dialect::<PostgreSQLDialect>(&data);
        tokens = quote! {#tokens #imple};
    }

    if data.sqlite {
        let imple = impl_delete_trait_for_dialect::<SqliteDialect>(&data);
        tokens = quote! {#tokens #imple};
    }

    tokens
}

fn impl_delete_trait_for_dialect<L>(data: &StructData) -> TokenStream
where
    L: SqlxConnectionType + HasDelete,
{
    let for_struct = &data.struct_name;

    let sql_statement = L::delete_by_unique_key(
        data.table.get_table_identifier(),
        data.table
            .get_primary_key()
            .expect("Missing primary key for delete"),
    );

    let sql = sql_statement.sql;
    let binds = generate_self_binds(&sql_statement.binds);
    let for_conn = L::get_mut_connection();

    let context = get_snafu_type(data, "Delete").map(|name| quote! {.context(#name)});
    let error = get_error_type(data);

    quote! {
        impl sequelles::Delete<#for_conn> for #for_struct {
            type Error = #error;

            async fn delete(&self, conn: #for_conn) -> Result<(), Self::Error> {
                use sequelles::snafu::ResultExt as _;

                sequelles::sqlx::query(#sql)
                    #binds
                    .execute(conn)
                    .await
                    #context?;
                Ok(())
            }
        }
    }
}
