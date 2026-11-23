use crate::models::dialects::PostgreSQLDialect;
use crate::models::dialects::SqliteDialect;
use crate::models::schema::field::field_collection::ColumnCollection;
use crate::models::schema::unique_key::UniqueKey;
use crate::models::sql_bind::HasBinds;
use crate::models::sql_bind::SqlBinds;
use crate::models::sql_statement::SqlStatement;

pub trait HasUpdate {
    fn update_by_unique_key(
        table_name: &str,
        fields_to_update: &ColumnCollection,
        unique_key: &UniqueKey,
    ) -> SqlStatement;
}

fn update_postgres_like<L>(
    table_name: &str,
    fields_to_update: &ColumnCollection,
    unique_key: &UniqueKey,
) -> SqlStatement
where
    L: HasBinds,
{
    let mut binds = SqlBinds::default();

    let updated_fields = fields_to_update.as_sql_update_fields::<PostgreSQLDialect>(&mut binds);
    let sql_where = unique_key.as_sql_where::<PostgreSQLDialect>(&mut binds);

    SqlStatement {
        sql: format!("UPDATE {table_name} SET {updated_fields} WHERE {sql_where} RETURNING *"),
        binds: binds,
    }
}

impl HasUpdate for PostgreSQLDialect {
    fn update_by_unique_key(
        table_name: &str,
        fields_to_update: &ColumnCollection,
        unique_key: &UniqueKey,
    ) -> SqlStatement {
        update_postgres_like::<Self>(table_name, fields_to_update, unique_key)
    }
}

impl HasUpdate for SqliteDialect {
    fn update_by_unique_key(
        table_name: &str,
        fields_to_update: &ColumnCollection,
        unique_key: &UniqueKey,
    ) -> SqlStatement {
        update_postgres_like::<Self>(table_name, fields_to_update, unique_key)
    }
}
