use itertools::Itertools;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use sequelles_sql_gen::models::dialects::PostgreSQLDialect;
use sequelles_sql_gen::models::dialects::SqliteDialect;
use syn::Ident;

use crate::models::database_data::StructData;
use crate::models::sql_dialects::SqlxConnectionType;
use crate::tables::dyn_insert_struct::CreateDynInsertableStruct;
use crate::tables::error::get_error_type;
use crate::tables::error::get_snafu_type;

impl CreateDynInsertableStruct {
    pub(super) fn create_impl_inserts(data: &StructData) -> TokenStream {
        if !data.gen_insert_struct {
            return quote! {};
        }

        let mut tokens = quote::quote! {};

        if data.postgres {
            let imple = create_impl_insert_for_dialect::<PostgreSQLDialect>(&data);
            tokens = quote! {#tokens #imple};
        }

        if data.sqlite {
            let imple = create_impl_insert_for_dialect::<SqliteDialect>(&data);
            tokens = quote! {#tokens #imple};
        }

        tokens
    }
}

fn create_impl_insert_for_dialect<L>(data: &StructData) -> TokenStream
where
    L: SqlxConnectionType,
{
    let for_struct = CreateDynInsertableStruct::get_ident(&data.struct_name);
    let output_struct = &data.struct_name;

    let for_conn = L::get_mut_connection();
    let builder = L::get_seq_query_builder();
    let for_db = L::get_db_struct();

    let context = get_snafu_type(data, "Insert").map(|name| quote! {.context(#name)});
    let error = get_error_type(data);

    let hard_columns = gen_hard_column_list(data);
    let hard_binds = gen_hard_column_binds(data);
    let column_checks = gen_default_column_checks(data);
    let table_name = data.table.get_table_identifier();

    quote! {
        impl sequelles::InsertOrIgnore<#for_conn> for #for_struct {
            type Output = #output_struct;
            type Error = #error;

            async fn insert_or_ignore(self, conn: #for_conn) -> Result<Option<Self::Output>, Self::Error> {
                use sequelles::snafu::ResultExt as _;
                use sequelles::sea_query_sqlx::SqlxBinder as _;

                let mut columns = #hard_columns;
                let mut binds: Vec<sequelles::sea_query::expr::Expr> = #hard_binds;

                #column_checks

                let (sql, binds) = sequelles::sea_query::Query::insert()
                    .into_table(#table_name)
                    .columns(columns)
                    .values_panic(binds)
                    .returning_all()
                    .build_sqlx(#builder);

                sequelles::sqlx::query_as_with::<#for_db, Self::Output, _>(sequelles::sqlx::AssertSqlSafe(sql), binds)
                    .fetch_optional(conn)
                    .await
                    #context
            }
        }
    }
}

fn gen_hard_column_list(data: &StructData) -> TokenStream {
    let list = data.table.iter_non_default_columns().map(|f| &f.sql_name).collect_vec();

    quote! {vec![ #(#list),* ]}
}

fn gen_hard_column_binds(data: &StructData) -> TokenStream {
    let list = data
        .table
        .iter_non_default_columns()
        .map(|f| {
            let rust_name = Ident::new(&f.rust_name, Span::call_site());

            quote! {self.#rust_name.into()}
        })
        .collect_vec();

    quote! {vec![ #(#list),* ]}
}

fn gen_default_column_checks(data: &StructData) -> TokenStream {
    data.table
        .iter_default_columns()
        .map(|col| {
            gen_default_column_check(
                &data
                    .get_field_by_name(&col.rust_name)
                    .unwrap()
                    .ident
                    .as_ref()
                    .unwrap(),
                &col.sql_name,
            )
        })
        .collect()
}

fn gen_default_column_check(rust_ident: &syn::Ident, sql_name: &str) -> TokenStream {
    quote! {
        if !self.#rust_ident.is_default() {
            columns.push(#sql_name);
            binds.push(self.#rust_ident.into_option().into())
        }
    }
}
