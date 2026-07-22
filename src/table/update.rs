pub trait Update<C>: Sized {
    type Output;
    type Error;

    /// Update the row in the database. 
    /// 
    /// This will not modify self to reflect the data returned by the query. If you need to keep the data synced, use [`Self::update_self`]
    fn update(&self, conn: C) -> impl Future<Output = Result<Option<Self::Output>, Self::Error>>;
}

pub trait UpdateSelf<C> {
    type Error;

    /// Update the row in the database, then if a row is returned, update self with the new data
    fn update_self(&mut self, conn: C)
    -> impl Future<Output = Result<bool, Self::Error>>;
}

impl<C, T> UpdateSelf<C> for T
where
    T: Update<C>,
    T::Output: Into<T>,
{
    type Error = T::Error;

    async fn update_self(&mut self, conn: C) -> Result<bool, Self::Error> {
        match self.update(conn).await? {
            Some(val) => {
                *self = val.into();
                Ok(true)
            }
            None => Ok(false),
        }
    }
}
