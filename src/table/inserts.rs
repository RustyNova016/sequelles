
pub trait InsertOrIgnore<C> {
    type Output;

    /// Insert the row into the database. If there's any constraint preventing the insert, ignore and returns none
    fn insert_or_ignore(
        &self,
        conn: C,
    ) -> impl Future<Output = Result<Option<Self::Output>, sqlx::Error>>;
}

pub trait InsertOrIgnoreSelf<C> {
    /// Insert the row into the database, and save the result into self. If there's any constraint preventing the insert, ignore and returns false
    fn insert_or_ignore_self(&mut self, conn: C)
    -> impl Future<Output = Result<bool, sqlx::Error>>;
}

impl<C, T> InsertOrIgnoreSelf<C> for T
where
    T: InsertOrIgnore<C>,
    T::Output: Into<T>,
{
    async fn insert_or_ignore_self(&mut self, conn: C) -> Result<bool, sqlx::Error> {
        match self.insert_or_ignore(conn).await? {
            Some(val) => {
                *self = val.into();
                Ok(true)
            }
            None => Ok(false),
        }
    }
}
