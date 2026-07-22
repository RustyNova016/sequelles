use proc_macro2::TokenStream;
use quote::quote;

pub struct Sqlite;

impl Sqlite {
    pub fn get_conn() -> TokenStream {
        quote! {&mut sequelles::sqlx::SqliteConnection}
    }
}

pub struct PostgreSQL;