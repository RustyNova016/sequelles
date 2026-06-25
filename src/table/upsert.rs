
pub trait Upsert<C> {
    type Output;

    /// Insert the row into the database. If there's any constraint preventing the insert, ignore and returns none
    fn upsert(
        &self,
        conn: C,
    ) -> impl Future<Output = Result<Option<Self::Output>, sqlx::Error>>;
}

pub trait UpsertSelf<C> {
    /// Insert the row into the database, and save the result into self. If there's any constraint preventing the insert, ignore and returns false
    fn upsert_self(&mut self, conn: C)
    -> impl Future<Output = Result<bool, sqlx::Error>>;
}

impl<C, T> UpsertSelf<C> for T
where
    T: Upsert<C>,
    T::Output: Into<T>,
{
    async fn upsert_self(&mut self, conn: C) -> Result<bool, sqlx::Error> {
        match self.upsert(conn).await? {
            Some(val) => {
                *self = val.into();
                Ok(true)
            }
            None => Ok(false),
        }
    }
}
