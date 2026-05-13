use core::fmt::Display;

use convert_case::Casing;
use proc_macro2::Span;
use syn::Ident;

use crate::tables::attributes::RelationAtribute;

#[derive(Debug)]
pub struct RelationData {
    pub references: String,
    pub with_row: Ident,
}

impl RelationData {
    pub fn parse(attr: RelationAtribute) -> Option<Self> {
        match (attr.references, attr.with_row) {
            (Some(related_to), Some(with_row)) => Some(Self {
                references: related_to,
                with_row,
            }),
            (None, None) => None,
            (Some(_), None) => panic!("Missing attibute `with_row`"),
            (None, Some(_)) => panic!("Missing attibute `references`"),
        }
    }

    pub fn get_relation_struct_name(&self, db_table: impl Display) -> Ident {
        Ident::new(
            &format!(
                "{}HasMany{}",
                self.with_row
                    .to_string()
                    .to_case(convert_case::Case::Pascal),
                db_table.to_string().to_case(convert_case::Case::Pascal)
            ),
            Span::call_site(),
        )
    }
}
