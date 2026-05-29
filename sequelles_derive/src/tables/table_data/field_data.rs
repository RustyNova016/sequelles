use quote::ToTokens;
use syn::Field;

use crate::tables::attributes::ColumnAtribute;
use crate::tables::table_data::relation::RelationData;

#[derive(Debug)]
pub struct FieldData {
    /// Field AST
    pub field: Field,

    /// Its name in the database
    pub db_name: String,

    pub auto_increment: bool,

    pub default: bool,

    /// UNIQUE INDEX
    pub uindexes: Vec<String>,

    pub relation: Option<RelationData>,
}

impl FieldData {
    pub fn parse(field: &mut Field) -> Self {
        let cattr: ColumnAtribute = deluxe::extract_attributes(field).unwrap();

        let db_name = cattr
            .db_name
            .or_else(|| field.ident.as_ref().map(|i| i.to_string()))
            .expect("Field should have an ident");

        let mut uindexes = Vec::new();
        if cattr.pk {
            uindexes.push("pk".to_string());
        }

        if cattr.unique {
            uindexes.push(db_name.clone());
        }

        let relation = RelationData::parse(cattr.relation);

        Self {
            field: field.to_owned(),
            db_name,
            uindexes,
            auto_increment: cattr.auto_increment,
            default: cattr.default,
            relation,
        }
    }
}

impl ToTokens for FieldData {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.field.to_tokens(tokens);
    }
}
