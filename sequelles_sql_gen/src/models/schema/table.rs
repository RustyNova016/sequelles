use std::rc::Rc;

use crate::models::schema::field::Column;
use crate::models::schema::field::field_collection::ColumnCollection;
use crate::models::schema::unique_key::UniqueKey;

pub struct Table {
    pub name: String,
    pub schema: Option<String>,
    pub fields: ColumnCollection,
    unique_keys: Vec<UniqueKey>,
}

impl Table {
    pub fn new(name: String, schema: Option<String>) -> Self {
        Self {
            name,
            schema,
            fields: ColumnCollection::default(),
            unique_keys: Vec::default(),
        }
    }

    pub fn get_table_identifier(&self) -> String {
        match &self.schema {
            Some(schema) => format!("{schema}.{}", self.name),
            None => self.name.clone(),
        }
    }

    pub fn get_primary_key(&self) -> Option<&UniqueKey> {
        self.unique_keys.iter().find(|key| key.is_pk)
    }

    pub fn get_unique_key_by_name(&self, name: &str) -> Option<&UniqueKey> {
        self.unique_keys.iter().find(|key| key.name == name)
    }

    pub fn get_unique_key_by_name_mut(&mut self, name: &str) -> Option<&mut UniqueKey> {
        self.unique_keys.iter_mut().find(|key| key.name == name)
    }

    pub fn add_unique_key(&mut self, key: UniqueKey) -> bool {
        if key.is_pk && self.get_primary_key().is_some() {
            return false;
        }

        if self.get_unique_key_by_name(&key.name).is_some() {
            return false;
        }

        self.unique_keys.push(key);
        true
    }

    pub fn selsert_key_mut(&mut self, name: &str, is_pk: bool) -> &UniqueKey {
        if self.get_unique_key_by_name(name).is_none() {
            let key = UniqueKey::new(name.to_string(), is_pk);
            self.add_unique_key(key);
        }

        self.get_unique_key_by_name(name).unwrap()
    }

    pub fn insert_field_for_key(&mut self, name: &str, field: &Rc<Column>, is_pk: bool) -> bool {
        if self.get_unique_key_by_name(name).is_none() {
            let key = UniqueKey::new(name.to_string(), is_pk);
            self.add_unique_key(key);
        }

        let key = self.get_unique_key_by_name_mut(name).unwrap();
        key.fields.push(field)
    }

    pub fn iter_keys(&self) -> std::slice::Iter<'_, UniqueKey> {
        self.unique_keys.iter()
    }

    pub fn unique_keys(&self) -> &Vec<UniqueKey> {
        &self.unique_keys
    }

    pub fn iter_columns_not_in_pk(&self) -> impl Iterator<Item = &Rc<Column>> {
        self.fields.iter().filter(|field| {
            self.get_primary_key()
                .is_some_and(|pk| pk.fields.as_vec().contains(*field))
        })
    }

    pub fn iter_default_columns(&self) -> impl Iterator<Item = &Rc<Column>> {
        self.fields.iter().filter(|f| f.auto_increment || f.default)
    }

    pub fn iter_non_default_columns(&self) -> impl Iterator<Item = &Rc<Column>> {
        self.fields
            .iter()
            .filter(|f| !f.auto_increment && !f.default)
    }
}
