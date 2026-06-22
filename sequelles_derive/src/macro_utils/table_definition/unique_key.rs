use std::collections::HashMap;
use std::rc::Rc;

use itertools::Itertools;

use crate::macro_utils::attributes::table_attributes::TableUniqueKey;
use crate::macro_utils::table_definition::field_data::FieldData;

#[derive(Debug, Default)]
pub struct UniqueKeys(pub HashMap<String, UniqueKeyData>);

impl UniqueKeys {
    pub fn add_field(&mut self, field: &Rc<FieldData>) {
        if field.primary_key {
            self.0
                .entry("primary_key".to_string())
                .or_insert_with(|| UniqueKeyData::new("primary_key".to_string()))
                .fields
                .push(field.clone());
        } else if field.unique {
            self.0
                .entry(field.ident().to_string())
                .or_insert_with(|| UniqueKeyData::new(field.ident().to_string()))
                .fields
                .push(field.clone());
        }
    }

    pub fn add_table_unique_key(
        &mut self,
        value: TableUniqueKey,
        fields: &HashMap<String, Rc<FieldData>>,
    ) {
        let key_name = match value.key_name {
            Some(name) => name,
            None => value.columns.iter().map(|c| c.to_string()).join("_"),
        };

        let entry = self
            .0
            .entry(key_name.to_string())
            .or_insert_with(|| UniqueKeyData::new(key_name.to_string()));

        for field in value.columns {
            let field = fields
                .get(&field.to_string())
                .expect(&format!(
                    "Couldn't find field {} for unique relation {}",
                    field.to_string(),
                    &key_name
                ))
                .clone();

            entry.fields.push(field);
        }
    }

    pub fn get(&self, name: &str) -> Option<&UniqueKeyData> {
        self.0.get(name)
    }

    pub fn get_pk(&self) -> Option<&UniqueKeyData> {
        self.0.get("primary_key")
    }
}

#[derive(Debug, Default)]
pub struct UniqueKeyData {
    pub name: String,

    pub fields: Vec<Rc<FieldData>>,
}

impl UniqueKeyData {
    pub fn new(name: String) -> Self {
        Self {
            name,
            fields: Vec::new(),
        }
    }

    pub fn fields_as_ref_iter(&self) -> impl Iterator<Item = &FieldData> {
        self.fields.iter().map(Rc::as_ref)
    }
}
