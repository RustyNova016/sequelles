use convert_case::Case;
use convert_case::Casing;
use proc_macro2::Ident;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use sequelles_sql_gen::models::dialects::PostgreSQLDialect;
use sequelles_sql_gen::models::dialects::SqliteDialect;
use sequelles_sql_gen::models::queries::select::HasSelectByKey;
use sequelles_sql_gen::models::schema::unique_key::UniqueKey;

use crate::macro_utils::generate_binds::generate_binds;
use crate::models::database_data::StructData;
use crate::models::sql_dialects::SqlxConnectionType;
use crate::tables::error::get_error_type;
use crate::tables::error::get_snafu_type;

pub fn impl_select_unique_trait(data: &StructData, ukey: &UniqueKey) -> TokenStream {
    if !data.gen_select_unique {
        return quote! {};
    }

    let mut tokens = quote! {};

    if data.postgres {
        let imple = impl_select_trait_for_dialect::<PostgreSQLDialect>(&data, ukey);
        tokens = quote! {#tokens #imple};
    }

    if data.sqlite {
        let imple = impl_select_trait_for_dialect::<SqliteDialect>(&data, ukey);
        tokens = quote! {#tokens #imple};
    }

    tokens
}

fn impl_select_trait_for_dialect<L>(data: &StructData, ukey: &UniqueKey) -> TokenStream
where
    L: SqlxConnectionType + HasSelectByKey,
{
    let for_struct = &data.struct_name;
    let filter_struct_name = Ident::new(
        &format!(
            "{}{}",
            data.struct_name,
            ukey.name.to_string().to_case(Case::Pascal)
        ),
        Span::call_site(),
    );

    let sql_statement =
        L::select_by_unique_key(&data.table.get_table_identifier(), &data.table.fields, ukey);

    let sql = sql_statement.sql;
    let binds = generate_binds(&quote! {filter}, &sql_statement.binds);
    let for_conn = L::get_mut_connection();

        let context = get_snafu_type(data, "SelectUnique").map(|name| quote! {.context(#name)});
    let error = get_error_type(data);

    quote! {
        impl sequelles::SelectUnique<#for_conn, #filter_struct_name> for #for_struct {
            type Output = Self;
            type Error = #error;

            async fn select_unique(
                conn: #for_conn,
                filter: #filter_struct_name,
            ) -> Result<Option<Self>, Self::Error> {
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
