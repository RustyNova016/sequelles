use core::fmt::Display;

use itertools::Itertools;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::tables::table_data::field_data::FieldData;

pub fn create_filter_struct(
    derived_struct: &Ident,
    table_name: impl Display,
    fields: &[&FieldData],
) -> TokenStream {
    // Struct
    let filter_struct_name = Ident::new(&format!("{derived_struct}Filter"), Span::call_site());

    let struct_fields = fields
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
    let select = impl_select(derived_struct, &filter_struct_name, table_name, fields);

    quote! {
        #[derive(sequelles::bon::Builder)]
        pub struct #filter_struct_name {
            #(#struct_fields),*
        }

        #select
    }
}

fn impl_select(
    for_struct: &Ident,
    filter_struct_name: &Ident,
    table_name: impl Display,
    fields: &[&FieldData],
) -> TokenStream {
    let table_name = table_name.to_string();

    let field_conds = fields
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
                    .column(sequelles::sea_query::Asterisk)
                    .from(#table_name)
                    .cond_where(cond)
                    .build_sqlx(sequelles::sea_query::SqliteQueryBuilder);

                sequelles::sqlx::query_as_with::<sequelles::sqlx::Sqlite, Self, _>(sequelles::sqlx::AssertSqlSafe(sql), binds)
                    .fetch_all(conn)
                    .await
            }
        }
    }
}
