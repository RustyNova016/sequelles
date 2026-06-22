use proc_macro2::TokenStream;
use quote::quote;
use sequelles_sql_gen::models::dialects::PostgreSQLDialect;
use sequelles_sql_gen::models::dialects::SqliteDialect;

pub trait SqlxConnectionType {
    fn get_db_struct() -> TokenStream;

    fn get_mut_connection() -> TokenStream;

    fn get_seq_query_builder() -> TokenStream;
}

impl SqlxConnectionType for PostgreSQLDialect {
    fn get_db_struct() -> TokenStream {
        quote! {sequelles::sqlx::Postgres}
    }

    fn get_mut_connection() -> TokenStream {
        quote! {&mut sequelles::sqlx::PgConnection}
    }

    fn get_seq_query_builder() -> TokenStream {
        quote! {sequelles::sea_query::PostgresQueryBuilder}
    }
}

impl SqlxConnectionType for SqliteDialect {
    fn get_db_struct() -> TokenStream {
        quote! {sequelles::sqlx::Sqlite}
    }

    fn get_mut_connection() -> TokenStream {
        quote! {&mut sequelles::sqlx::SqliteConnection}
    }

    fn get_seq_query_builder() -> TokenStream {
        quote! {sequelles::sea_query::SqliteQueryBuilder}
    }
}
