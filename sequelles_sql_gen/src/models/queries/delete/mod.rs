use core::fmt::Display;

use crate::models::dialects::PostgreSQLDialect;
use crate::models::dialects::SqliteDialect;
use crate::models::schema::unique_key::UniqueKey;
use crate::models::sql_bind::HasBinds;
use crate::models::sql_bind::SqlBinds;
use crate::models::sql_statement::SqlStatement;

pub trait HasDelete {
    fn delete_by_unique_key(table_name: impl Display, unique_key: &UniqueKey) -> SqlStatement;
}

fn delete_postgres_like<L>(table_name: impl Display, unique_key: &UniqueKey) -> SqlStatement
where
    L: HasBinds,
{
    let mut binds = SqlBinds::default();

    let sql_where = unique_key.as_sql_where::<L>(&mut binds);

    SqlStatement {
        sql: format!("DELETE FROM {table_name} WHERE {sql_where}",),
        binds: binds,
    }
}

impl HasDelete for PostgreSQLDialect {
    fn delete_by_unique_key(table_name: impl Display, unique_key: &UniqueKey) -> SqlStatement {
        delete_postgres_like::<Self>(table_name, unique_key)
    }
}

impl HasDelete for SqliteDialect {
    fn delete_by_unique_key(table_name: impl Display, unique_key: &UniqueKey) -> SqlStatement {
        delete_postgres_like::<Self>(table_name, unique_key)
    }
}
