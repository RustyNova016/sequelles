use crate::JoinRelation;
use crate::ManyToManyJoin;
use crate::ManyToZeroJoin;
use crate::has_rowid::HasRowID;

/// A collection of [`JoinRelation`]
#[derive(Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct JoinCollection<T> {
    joins: Vec<JoinRelation<T>>,
}

impl<R> JoinCollection<R> {
    pub fn joins(&self) -> &Vec<JoinRelation<R>> {
        &self.joins
    }

    pub fn joins_mut(&mut self) -> &mut Vec<JoinRelation<R>> {
        &mut self.joins
    }

    pub fn push(&mut self, join: JoinRelation<R>) {
        self.joins.push(join);
    }

    /// See [`Vec::len`]
    pub fn len(&self) -> usize {
        self.joins.len()
    }

    /// See [`Vec::is_empty`]
    pub fn is_empty(&self) -> bool {
        self.joins.is_empty()
    }

    /// Convert the join relations into a [`ManyToZeroJoin`].
    ///
    /// This takes in a vec of the left values, then associate the right values contained in the [`JoinRelation`],
    /// using the [`JoinRelation::external_id`] field
    pub fn into_many_to_zero<L, T>(self, left_elements: T) -> ManyToZeroJoin<L, R>
    where
        L: HasRowID,
        T: IntoIterator<Item = L>,
    {
        let mut smart_join = ManyToZeroJoin::default();

        // Insert the left values
        for left in left_elements {
            smart_join.insert(left, None);
        }

        // Now add the right values
        for (l_id, right) in self.joins.into_iter().map(|join| join.into_tuple()) {
            smart_join.replace_by_id(l_id, right);
        }

        smart_join
    }

    pub fn into_many_to_many<L, T>(self, left_elements: T) -> ManyToManyJoin<L, R>
    where
        L: HasRowID,
        R: HasRowID,
        T: IntoIterator<Item = L>,
    {
        let mut smart_join = ManyToManyJoin::default();

        // Insert the left values
        for left in left_elements {
            smart_join.add_left(left);
        }

        for (l_id, right) in self.joins.into_iter().map(|join| join.into_tuple()) {
            smart_join.add_relation_ids(l_id, right.rowid());
            smart_join.add_right(right);
        }

        smart_join
    }
}

impl<T> From<Vec<JoinRelation<T>>> for JoinCollection<T> {
    fn from(value: Vec<JoinRelation<T>>) -> Self {
        Self { joins: value }
    }
}
