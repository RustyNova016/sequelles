pub mod sql_dialect;
use std::collections::HashMap;
use std::rc::Rc;

use itertools::Itertools;
use syn::DeriveInput;
use syn::Ident;

use crate::macro_utils::attributes::table_attributes::TableAtribute;
use crate::macro_utils::attributes::table_attributes::TableUniqueKey;
use crate::macro_utils::table_definition::field_data::FieldData;
use crate::macro_utils::table_definition::relation::RelationData;
use crate::macro_utils::table_definition::unique_key::UniqueKeyData;
use crate::macro_utils::table_definition::unique_key::UniqueKeys;

pub mod field_data;
pub mod unique_key;
pub mod relation;
pub mod key;

#[derive(Debug)]
pub struct TableData {
    pub struct_ident: Ident,
    pub db_name: String,

    pub fields: HashMap<String, Rc<FieldData>>,
    pub unique_keys: UniqueKeys,
    pub foreign_keys: Vec<RelationData>
}

impl TableData {
    pub fn parse(ast: &mut DeriveInput) -> Self {
        let struct_ident = ast.ident.clone();

        let tattr: TableAtribute = deluxe::extract_attributes(ast).unwrap();

        let syn::Data::Struct(ast) = &mut ast.data else {
            panic!("The derive macro should be on a struct");
        };

        let db_name = tattr
            .db_name
            .as_ref()
            .cloned()
            .unwrap_or_else(|| struct_ident.to_string());

        let mut fields = HashMap::new();
        for field in ast.fields.iter_mut() {
            let field = Rc::new(FieldData::parse(field));
            fields.insert(field.ident().to_string(), field);
        }

        let mut unique_keys = Self::parse_unique_keys(&fields, tattr.unique_keys);
        Self::parse_primary_key(&mut unique_keys, &fields, tattr.primary_key);

        Self {
            struct_ident,
            fields,
            db_name,
            unique_keys,
            foreign_keys: Vec::new(),
        }
    }

    fn parse_unique_keys(
        fields: &HashMap<String, Rc<FieldData>>,
        unique_keys: Vec<TableUniqueKey>,
    ) -> UniqueKeys {
        let mut output = UniqueKeys::default();

        // First add the unique fields
        for field in fields.values() {
            output.add_field(field);
        }

        // Multi column keys
        for key in unique_keys {
            output.add_table_unique_key(key, fields);
        }

        return output;
    }

    fn parse_primary_key(
        keys: &mut UniqueKeys,
        fields: &HashMap<String, Rc<FieldData>>,
        pk_idents: Vec<Ident>,
    ) {
        let key = TableUniqueKey {
            columns: pk_idents,
            key_name: Some("primary_key".to_string()),
        };

        keys.add_table_unique_key(key, fields);
    }
}
