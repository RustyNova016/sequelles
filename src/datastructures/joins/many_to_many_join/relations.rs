use crate::ManyToManyJoin;
use crate::has_rowid::HasRowID;

impl<L, R> ManyToManyJoin<L, R>
where
    L: HasRowID,
    R: HasRowID,
{
    // === Deletions ===

    /// Remove a relation between a left element and a right element using their rowids
    pub fn remove_relation_ids(&mut self, left: i64, right: i64) {
        if let Some(left_vec) = self.left_to_right.get_mut(&left) {
            left_vec.retain(|&x| x != right);
        }
        if let Some(right_vec) = self.right_to_left.get_mut(&right) {
            right_vec.retain(|&x| x != left);
        }
    }

    /// Remove a relation between a left element and a right element
    pub fn remove_relation(&mut self, left: &L, right: &R) {
        self.remove_relation_ids(left.rowid(), right.rowid());
    }
}
