use sqlx::FromRow;

/// Represent a row of a table, with an associated external table rowid.
///
/// For exemple: the underlying data is a row from the `recordings` table, and the associated id is the id of artist of the recording
#[derive(Clone, PartialEq, Eq, Hash, Debug, FromRow)]
pub struct JoinRelation<T> {
    /// The id of the external row associated to it
    pub original_id: i64,

    /// The table's row
    #[sqlx(flatten)]
    pub data: T,
}

impl<T> JoinRelation<T> {
    pub fn new(data: T, original_id: i64) -> Self {
        Self { data, original_id }
    }

    /// Convert the join relation into a tuple
    pub fn into_tuple(self) -> (i64, T) {
        (self.original_id, self.data)
    }
}
