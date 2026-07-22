use quote::ToTokens;
use syn::Field;
use syn::Ident;

use crate::macro_utils::attributes::field_attributes::ColumnAtribute;

#[derive(Debug)]
pub struct FieldData {
    /// Field AST
    pub field: Field,

    /// Its name in the database
    pub db_name: String,

    pub auto_increment: bool,

    pub default: bool,

    pub primary_key: bool,
    pub unique: bool,
    //pub relation: Option<RelationData>,
}

impl FieldData {
    pub fn parse(field: &mut Field) -> Self {
        let cattr: ColumnAtribute = deluxe::extract_attributes(field).unwrap();

        let db_name = cattr
            .db_name
            .or_else(|| field.ident.as_ref().map(|i| i.to_string()))
            .expect("Cannot derive a tupple struct");

        //let relation = RelationData::parse(cattr.relation);

        Self {
            field: field.to_owned(),
            db_name,
            auto_increment: cattr.auto_increment,
            default: cattr.default,
            //relation,
            primary_key: cattr.primary_key,
            unique: cattr.unique,
        }
    }

    pub fn ident(&self) -> &Ident {
        &self
            .field
            .ident
            .as_ref()
            .expect("Cannot derive a tupple struct")
    }
}

impl ToTokens for FieldData {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.field.to_tokens(tokens);
    }
}
