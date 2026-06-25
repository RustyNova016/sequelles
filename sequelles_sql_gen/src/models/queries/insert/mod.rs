use crate::models::dialects::PostgreSQLDialect;
use crate::models::dialects::SqliteDialect;
use crate::models::schema::field::field_collection::ColumnCollection;
use crate::models::sql_bind::HasBinds;
use crate::models::sql_bind::SqlBinds;
use crate::models::sql_statement::SqlStatement;

pub trait HasInsert {
    fn insert(
        table_name: &str,
        fields_to_insert: &ColumnCollection,
        ignore_conflict: bool,
    ) -> SqlStatement;
}

fn insert_postgres_like<L>(
    table_name: &str,
    fields_to_insert: &ColumnCollection,
    ignore_conflict: bool,
) -> SqlStatement
where
    L: HasBinds,
{
    let mut binds = SqlBinds::default();

    let fields = fields_to_insert.as_sql_field_list();
    let values = fields_to_insert.as_insert_binds::<L>(&mut binds);

    let conflict = if ignore_conflict {
        "ON CONFLICT DO NOTHING"
    } else {
        ""
    };

    SqlStatement {
        sql: format!("INSERT INTO {table_name} ({fields}) VALUES ({values}) {conflict} RETURNING *"),
        binds: binds,
    }
}

impl HasInsert for PostgreSQLDialect {
    fn insert(
        table_name: &str,
        fields_to_insert: &ColumnCollection,
        ignore_conflict: bool,
    ) -> SqlStatement {
        insert_postgres_like::<Self>(table_name, fields_to_insert, ignore_conflict)
    }
}

impl HasInsert for SqliteDialect {
    fn insert(
        table_name: &str,
        fields_to_insert: &ColumnCollection,
        ignore_conflict: bool,
    ) -> SqlStatement {
        insert_postgres_like::<Self>(table_name, fields_to_insert, ignore_conflict)
    }
}
