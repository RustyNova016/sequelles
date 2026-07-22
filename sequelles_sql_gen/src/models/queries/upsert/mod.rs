use itertools::Itertools;

use crate::models::dialects::PostgreSQLDialect;
use crate::models::dialects::SqliteDialect;
use crate::models::schema::field::field_collection::ColumnCollection;
use crate::models::schema::unique_key::UniqueKey;
use crate::models::sql_bind::SqlBinds;
use crate::models::sql_statement::SqlStatement;

pub trait HasUpsert {
    fn upsert(
        table_name: &str,
        fields_to_insert: &ColumnCollection,
        fields_to_update: &ColumnCollection,
        conflict_keys: &[UniqueKey],
    ) -> SqlStatement;
}

impl HasUpsert for PostgreSQLDialect {
    fn upsert(
        table_name: &str,
        fields_to_insert: &ColumnCollection,
        fields_to_update: &ColumnCollection,
        conflict_keys: &[UniqueKey],
    ) -> SqlStatement {
        let mut binds = SqlBinds::default();

        let fields = fields_to_insert.as_sql_field_list();
        let values = fields_to_insert.as_insert_binds::<PostgreSQLDialect>(&mut binds);

        let conflict = conflict_keys
            .iter()
            .map(|key| format!("({})", key.as_sql_where::<PostgreSQLDialect>(&mut binds)))
            .join(" OR ");

        let updated_fields = fields_to_update.as_sql_update_fields::<PostgreSQLDialect>(&mut binds);
        let insert_fields = fields_to_insert.as_sql_field_list_with_prefix("source");

        let sql = format!(
            "
            MERGE INTO {table_name} as target
            USING (VALUES({values})) AS 
                source({fields})
            ON {conflict}
            WHEN MATCHED THEN
                UPDATE SET {updated_fields}
            WHEN NOT MATCHED THEN
                INSERT ({fields}) 
                VALUES ({insert_fields})
        "
        );

        SqlStatement { sql, binds }
    }
}

impl HasUpsert for SqliteDialect {
    fn upsert(
        table_name: &str,
        fields_to_insert: &ColumnCollection,
        fields_to_update: &ColumnCollection,
        conflict_keys: &[UniqueKey],
    ) -> SqlStatement {
        let mut binds = SqlBinds::default();

        let fields = fields_to_insert.as_sql_field_list();
        let values = fields_to_insert.as_insert_binds::<SqliteDialect>(&mut binds);

        let conflict_update = fields_to_update
            .iter()
            .map(|f| format!("{} = EXCLUDED.{}", f.sql_name, f.sql_name))
            .join(", ");
        let conflict_clauses = conflict_keys
            .iter()
            .map(|key| {
                format!(
                    "ON CONFLICT ({}) DO UPDATE SET {conflict_update}",
                    key.fields.as_sql_field_list()
                )
            })
            .join(" ");

        SqlStatement {
            sql: format!(
                "INSERT INTO {table_name} ({fields}) VALUES ({values}) {conflict_clauses} RETURNING *"
            ),
            binds: binds,
        }
    }
}
