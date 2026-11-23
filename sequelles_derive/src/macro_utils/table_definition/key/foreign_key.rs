use std::rc::Rc;

use crate::macro_utils::table_definition::field_data::FieldData;

#[derive(Debug, Default)]
pub struct ForeignKey {
    pub columns: Vec<ForeignKeyColumn>,

    pub foreign_table: String,
}



#[derive(Debug)]
pub struct ForeignKeyColumn {
    pub local_column: Rc<FieldData>,

    pub foreign_column: String,
    pub foreign_field: String,
}