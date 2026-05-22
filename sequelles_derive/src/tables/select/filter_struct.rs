use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::quote;

use crate::tables::table_data::TableData;

pub fn create_filter_struct(table_data: &TableData) -> TokenStream {
    // Struct
    let filter_struct_name = table_data.get_filter_struct_name();

    let struct_fields = table_data
        .fields
        .iter()
        .map(|f| {
            let name = &f.field.ident;
            let typ = &f.field.ty;

            quote! {
                #[builder(into)]
                pub #name: Option<#typ>
            }
        })
        .collect_vec();

    // Impl
    let select = impl_select(&table_data);

    quote! {
        #[derive(sequelles::bon::Builder)]
        pub struct #filter_struct_name {
            #(#struct_fields),*
        }

        #select
    }
}

fn impl_select(table_data: &TableData) -> TokenStream {
    let for_struct = &table_data.struct_ident;
    let db_name = &table_data.db_name;
    let filter_struct_name = table_data.get_filter_struct_name();

    let field_conds = table_data
        .fields
        .iter()
        .map(|f| {
            let field_ident = &f.field.ident;
            let db_name = &f.db_name;

            quote! {
                if let Some(value) = filter.#field_ident{
                    cond = cond.add(sequelles::sea_query::Expr::col(#db_name).eq(value))
                }
            }
        })
        .collect_vec();

    quote! {
        impl sequelles::Select<&mut sequelles::sqlx::SqliteConnection, #filter_struct_name> for #for_struct {
            async fn select(
                conn: &mut sequelles::sqlx::SqliteConnection,
                filter: #filter_struct_name,
            ) -> Result<Vec<Self>, sqlx::Error> {
                use sequelles::sea_query::ExprTrait as _;
                use sequelles::sea_query_sqlx::SqlxBinder as _;

                let mut cond = sequelles::sea_query::Cond::all();

                #(#field_conds)*

                let (sql, binds) = sequelles::sea_query::Query::select()
                    .from(#db_name)
                    .cond_where(cond)
                    .build_sqlx(sequelles::sea_query::SqliteQueryBuilder);

                sequelles::sqlx::query_as_with::<Sqlite, Self, _>(&sql, binds)
                    .fetch_all(conn)
                    .await
            }
        }
    }
}
