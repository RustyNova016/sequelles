use itertools::Itertools;
use proc_macro2::Ident;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use sequelles_sql_gen::models::dialects::PostgreSQLDialect;
use sequelles_sql_gen::models::dialects::SqliteDialect;

use crate::models::database_data::StructData;
use crate::models::sql_dialects::SqlxConnectionType;
use crate::tables::select::filter_struct::create_filter_struct;

pub(super) mod filter_struct;

pub fn impl_select_trait(data: &StructData) -> TokenStream {
    if !data.gen_select {
        return quote! {};
    }

    let mut tokens = create_filter_struct(data);

    if data.postgres {
        let imple = impl_select_trait_for_dialect::<PostgreSQLDialect>(&data);
        tokens = quote! {#tokens #imple};
    }

        if data.sqlite {
        let imple = impl_select_trait_for_dialect::<SqliteDialect>(&data);
        tokens = quote! {#tokens #imple};
    }


    tokens
}

fn impl_select_trait_for_dialect<L>(data: &StructData) -> TokenStream
where
    L: SqlxConnectionType,
{
    let for_struct = &data.struct_name;
    let for_db = L::get_db_struct();
    let for_conn = L::get_mut_connection();
    let builder = L::get_seq_query_builder();
    let table_name = data.table.get_table_identifier();
    let filter_struct_name = Ident::new(&format!("{}Filter", data.struct_name), Span::call_site());

    let field_conds = data
        .table
        .fields
        .iter()
        .map(|f| {
            let field_ident = Ident::new(&f.rust_name, Span::call_site());
            let db_name = &f.sql_name;

            quote! {
                if let Some(value) = filter.#field_ident{
                    cond = cond.add(sequelles::sea_query::Expr::col(#db_name).eq(value))
                }
            }
        })
        .collect_vec();

    quote! {
        impl sequelles::Select<#for_conn, #filter_struct_name> for #for_struct {
            async fn select(
                conn: #for_conn,
                filter: #filter_struct_name,
            ) -> Result<Vec<Self>, sqlx::Error> {
                use sequelles::sea_query::ExprTrait as _;
                use sequelles::sea_query_sqlx::SqlxBinder as _;

                let mut cond = sequelles::sea_query::Cond::all();

                #(#field_conds)*

                let (sql, binds) = sequelles::sea_query::Query::select()
                    .column(sequelles::sea_query::Asterisk)
                    .from(#table_name)
                    .cond_where(cond)
                    .build_sqlx(#builder);

                sequelles::sqlx::query_as_with::<#for_db, Self, _>(sequelles::sqlx::AssertSqlSafe(sql), binds)
                    .fetch_all(conn)
                    .await
            }
        }
    }
}
