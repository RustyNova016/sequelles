pub trait Delete<C> {
    /// Delete the row in the database.
    fn delete(&self, conn: C) -> impl Future<Output = Result<(), sqlx::Error>>;
}

