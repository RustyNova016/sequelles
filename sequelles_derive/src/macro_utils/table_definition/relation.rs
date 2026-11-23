#[derive(Debug, Default)]
pub struct RelationData {
    pub columns: Vec<RelatedColumn>,

    pub foreign_table: String,
}

impl RelationData {
    pub fn add_column(&mut self, column: RelatedColumn) {
        if self.columns.iter().any(|c| c.current_db == column.current_db) {
            panic!("Error: Column {} has multiple references", column.current_db)
        }

        self.columns.push(column);
    }
}

#[derive(Debug, Default)]
pub struct RelatedColumn {
    pub current_db: String,
    pub foreign_db: String
}