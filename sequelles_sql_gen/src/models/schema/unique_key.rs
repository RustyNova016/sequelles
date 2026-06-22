use crate::models::schema::field::field_collection::ColumnCollection;
use crate::models::sql_bind::HasBinds;
use crate::models::sql_bind::SqlBinds;

#[derive(Debug)]
pub struct UniqueKey {
    pub name: String,
    pub fields: ColumnCollection,

    pub is_pk: bool,
}

impl UniqueKey {
    pub fn new(name: String, is_pk: bool) -> Self {
        Self {
            is_pk,
            name,
            fields: ColumnCollection::default(),
        }
    }

    pub fn as_sql_where<L>(&self, binds: &mut SqlBinds) -> String
    where
        L: HasBinds,
    {
        self.fields.as_sql_where::<L>(binds)
    }
}
