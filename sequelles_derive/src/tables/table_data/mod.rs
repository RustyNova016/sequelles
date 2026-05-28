use itertools::Itertools;
use proc_macro2::Ident;
use proc_macro2::Span;
use syn::DeriveInput;

use crate::tables::attributes::table_attribute::TableAtribute;
use crate::tables::table_data::field_data::FieldData;

pub mod field_data;
pub mod unique_index;
pub mod relation;

#[derive(Debug)]
pub struct TableData {
    pub struct_ident: Ident,
    pub db_name: String,

    pub fields: Vec<FieldData>,
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

        let fields = ast
            .fields
            .iter_mut()
            .map(|field| FieldData::parse(field))
            .collect_vec();

        Self {
            struct_ident,
            fields,
            db_name,
        }
    }

    pub fn get_fields_with_uindex(&self, uindex: &str) -> impl Iterator<Item = &FieldData> {
        self.fields
            .iter()
            .filter(|f| f.uindexes.contains(&uindex.to_string()))
    }

    /// Create a where filter for a specific unique index
    pub fn get_unique_index_cond_sql(&self, uindex: &str) -> String {
        format!(
            "({})",
            self.get_fields_with_uindex(uindex)
                .map(|field| {
                    let rust_name = field
                        .field
                        .ident
                        .as_ref()
                        .map(|i| i.to_string())
                        .expect("Field should have an ident");

                    format!("`{}` = {{{rust_name}}}", &field.db_name)
                })
                .join(", ")
        )
    }

    pub fn get_uindex_names(&self) -> impl Iterator<Item = &String> {
        self.fields.iter().flat_map(|f| f.uindexes.iter()).unique()
    }
}
