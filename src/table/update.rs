pub trait Update<C> {
    /// Update the row in the database. This may modify the current struct to match the row after update
    fn update(&mut self, conn: C) -> impl Future<Output = Result<(), sqlx::Error>>;
}
