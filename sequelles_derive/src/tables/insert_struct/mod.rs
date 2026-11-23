pub mod builder_ext;
pub mod from_row;
use proc_macro2::Ident;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use sequelles_sql_gen::models::dialects::PostgreSQLDialect;
use sequelles_sql_gen::models::dialects::SqliteDialect;
use sequelles_sql_gen::models::queries::insert::HasInsert;

use crate::macro_utils::generate_binds::generate_self_binds;
use crate::models::database_data::StructData;
use crate::models::sql_dialects::SqlxConnectionType;
use crate::tables::error::get_error_type;
use crate::tables::error::get_snafu_type;
use crate::tables::insert_struct::create_struct::create_insert_struct;

pub mod create_struct;
pub mod impl_get_key;
pub mod selsert;

pub fn impl_insert_structs(data: &StructData) -> TokenStream {
    if !data.gen_insert_struct {
        return quote! {};
    }

    let mut tokens = create_insert_struct(data);

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
    let for_struct = Ident::new(&format!("{}Insert", &data.struct_name), Span::call_site());
    let row_struct = &data.struct_name;

    let sql_statement = L::insert(&data.table.get_table_identifier(), &data.table.fields, true);

    let sql = sql_statement.sql;
    let binds = generate_self_binds(&sql_statement.binds);
    let for_conn = L::get_mut_connection();

    let context = get_snafu_type(data, "Insert").map(|name| quote! {.context(#name)});
    let error = get_error_type(data);

    quote! {
        impl sequelles::InsertOrIgnore<#for_conn> for &#for_struct {
            type Output = #row_struct;
            type Error = #error;

            async fn insert_or_ignore(self, conn: #for_conn) -> Result<Option<#row_struct>, Self::Error> {
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
