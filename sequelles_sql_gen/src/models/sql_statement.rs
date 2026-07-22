use crate::models::sql_bind::SqlBinds;

pub struct SqlStatement {
    pub sql: String,
    pub binds: SqlBinds,
}
