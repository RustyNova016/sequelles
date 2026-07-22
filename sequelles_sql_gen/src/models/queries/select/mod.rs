use crate::models::dialects::PostgreSQLDialect;
use crate::models::dialects::SqliteDialect;
use crate::models::schema::field::field_collection::ColumnCollection;
use crate::models::schema::unique_key::UniqueKey;
use crate::models::sql_bind::HasBinds;
use crate::models::sql_bind::SqlBinds;
use crate::models::sql_statement::SqlStatement;

pub trait HasSelectByKey {
    fn select_by_unique_key(
        table_name: &str,
        fields: &ColumnCollection,
        unique_key: &UniqueKey,
    ) -> SqlStatement;
}

fn select_by_unique_key_postgres_like<L>(
    table_name: &str,
    fields: &ColumnCollection,
    unique_key: &UniqueKey,
) -> SqlStatement
where
    L: HasBinds,
{
    let mut binds = SqlBinds::default();

    let field_list = fields.as_sql_field_list();
    let sql_where = unique_key.as_sql_where::<PostgreSQLDialect>(&mut binds);

    SqlStatement {
        sql: format!("SELECT {field_list} FROM {table_name} WHERE {sql_where}"),
        binds: binds,
    }
}

impl HasSelectByKey for PostgreSQLDialect {
    fn select_by_unique_key(
        table_name: &str,
        fields: &ColumnCollection,
        unique_key: &UniqueKey,
    ) -> SqlStatement
    {
        select_by_unique_key_postgres_like::<Self>(table_name, fields, unique_key)
    }
}

impl HasSelectByKey for SqliteDialect {
    fn select_by_unique_key(
        table_name: &str,
        fields: &ColumnCollection,
        unique_key: &UniqueKey,
    ) -> SqlStatement
    {
        select_by_unique_key_postgres_like::<Self>(table_name, fields, unique_key)
    }
}



#[cfg(test)]
mod tests {
    use crate::models::dialects::PostgreSQLDialect;
    use crate::models::queries::select::HasSelectByKey as _;
    use crate::tests::fixtures::pies::fixture_pies;

    #[test]
    fn postgres_select_pies() {
        let table = fixture_pies();
        let sql = PostgreSQLDialect::select_by_unique_key(
            &table.name,
            &table.fields,
            table.get_primary_key().unwrap(),
        );

        assert_eq!(sql.sql, "SELECT id, name FROM pies WHERE id = $1");
    }
}
