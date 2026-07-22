use crate::models::sql_bind::HasBinds;
use crate::models::sql_bind::SqlBinds;

pub mod field_collection;

/// A database field
#[derive(Debug, PartialEq, Eq)]
pub struct Column {
    pub sql_name: String,
    pub rust_name: String,

    pub auto_increment: bool,
    //pub not_null: bool,
    pub default: bool,
}

impl Column {
    pub fn as_field_eq_bind<L>(&self, binds: &mut SqlBinds) -> String
    where
        L: HasBinds,
    {
        let bind = L::create_bind(binds, self.rust_name.clone());
        format!("{} = {bind}", self.sql_name)
    }
}
