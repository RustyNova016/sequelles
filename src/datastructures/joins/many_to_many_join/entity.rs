use crate::ManyToManyJoin;
use crate::has_rowid::HasRowID;

impl<L, R> ManyToManyJoin<L, R>
where
    L: HasRowID,
    R: HasRowID,
{
    // === Deletions ===

    /// Remove a row from the left table entirely from the join using its id
    pub fn remove_left_row_id(&mut self, id: &i64) -> Option<L> {
        self.left_to_right.remove_entry(id);
        self.right_to_left
            .iter_mut()
            .for_each(|(_, vals)| vals.retain(|val| val != id));
        self.left_table.remove(id)
    }

    /// Remove a row from the right table entirely from the join using its id
    pub fn remove_right_row_id(&mut self, id: &i64) -> Option<R> {
        self.right_to_left.remove_entry(id);
        self.left_to_right
            .iter_mut()
            .for_each(|(_, vals)| vals.retain(|val| val != id));
        self.right_table.remove(id)
    }

    /// Remove a row from the left table entirely from the join
    pub fn remove_left_row(&mut self, val: &L) -> Option<L> {
        self.remove_left_row_id(&val.rowid())
    }

    /// Remove a row from the right table entirely from the join
    pub fn remove_right_row(&mut self, val: &R) -> Option<R> {
        self.remove_right_row_id(&val.rowid())
    }
}
