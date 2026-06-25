use core::fmt::Display;
use std::rc::Rc;

use itertools::Itertools;

use crate::models::schema::field::Column;
use crate::models::sql_bind::HasBinds;
use crate::models::sql_bind::SqlBinds;

/// Holds multiple fields
#[derive(Debug, Default)]
pub struct ColumnCollection(Vec<Rc<Column>>);

impl ColumnCollection {
    pub fn get_field_with_sql_name(&self, name: &str) -> Option<&Rc<Column>> {
        self.0.iter().find(|f| f.sql_name == name)
    }

    pub fn get_field_with_rust_name(&self, name: &str) -> Option<&Rc<Column>> {
        self.0.iter().find(|f| f.rust_name == name)
    }

    /// Add a field to the collection. Will return false if it already exists
    pub fn push(&mut self, field: &Rc<Column>) -> bool {
        if self.get_field_with_sql_name(&field.sql_name).is_none()
            && self.get_field_with_rust_name(&field.rust_name).is_none()
        {
            self.0.push(field.clone());
            true
        } else {
            false
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Rc<Column>> {
        self.0.iter()
    }

    pub fn as_sql_field_list(&self) -> String {
        self.iter().map(|f| &f.sql_name).join(", ")
    }

    pub fn as_sql_field_list_with_prefix(&self, prefix: impl Display) -> String {
        self.iter()
            .map(|f| format!("{prefix}.{}", f.sql_name))
            .join(", ")
    }

    pub fn as_sql_update_fields<L>(&self, binds: &mut SqlBinds) -> String
    where
        L: HasBinds,
    {
        self.iter()
            .map(|f| f.as_field_eq_bind::<L>(binds))
            .join(", ")
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn as_sql_where<L>(&self, binds: &mut SqlBinds) -> String
    where
        L: HasBinds,
    {
        self.iter()
            .map(|f| f.as_field_eq_bind::<L>(binds))
            .join(" AND ")
    }

    pub fn as_insert_binds<L>(&self, binds: &mut SqlBinds) -> String
    where
        L: HasBinds,
    {
        self.iter()
            .map(|f| L::create_bind(binds, f.rust_name.clone()))
            .join(", ")
    }

    pub fn as_vec(&self) -> &Vec<Rc<Column>> {
        &self.0
    }
}

impl FromIterator<Rc<Column>> for ColumnCollection {
    fn from_iter<T: IntoIterator<Item = Rc<Column>>>(iter: T) -> Self {
        Self(iter.into_iter().collect_vec())
    }
}
