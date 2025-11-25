use crate::ManyToManyJoin;
use crate::has_rowid::HasRowID;

impl<L, R> ManyToManyJoin<L, R>
where
    L: HasRowID,
    R: HasRowID,
{
    pub fn relations(&self) -> Vec<(&L, &R)> {
        let mut relations = Vec::with_capacity(self.left_table.len() + self.right_table.len());

        for (left, rights) in &self.left_to_right {
            for right in rights {
                relations.push((
                    self.get_left(left).expect("Id should be in the map"),
                    self.get_right(right).expect("Id should be in the map"),
                ));
            }
        }

        relations
    }
}
