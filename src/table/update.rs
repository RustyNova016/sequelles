pub trait Update<C>: Sized {
    /// Update the row in the database. 
    /// 
    /// This will not modify self to reflect the data returned by the query. If you need to keep the data synced, use [`Self::update_self`]
    fn update(&self, conn: C) -> impl Future<Output = Result<Option<Self>, sqlx::Error>>;

    /// Update the row in the database, then if a row is returned, update self with the new data
    fn update_self(&mut self, conn: C) -> impl Future<Output = Result<(), sqlx::Error>> {
        async {
            match self.update(conn).await? {
                Some(val) => *self = val,
                None => {}
            }

            Ok(())
        }
    }
}
