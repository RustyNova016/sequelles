pub trait InsertOrIgnore<C> {
    type Output;
    type Error;

    /// Insert the row into the database. If there's any constraint preventing the insert, ignore and returns none
    fn insert_or_ignore(
        self,
        conn: C,
    ) -> impl Future<Output = Result<Option<Self::Output>, Self::Error>>;
}

pub trait InsertOrIgnoreSelf<C> {
    type Error;

    /// Insert the row into the database, and save the result into self. If there's any constraint preventing the insert, ignore and returns false
    fn insert_or_ignore_self(&mut self, conn: C)
    -> impl Future<Output = Result<bool, Self::Error>>;
}

impl<C, Row, T, E> InsertOrIgnoreSelf<C> for T
where
    for<'a> &'a T: InsertOrIgnore<C, Output = Row, Error = E>,
    T: From<Row>,
{
    type Error = E;

    async fn insert_or_ignore_self(&mut self, conn: C) -> Result<bool, Self::Error> {
        match self.insert_or_ignore(conn).await? {
            Some(val) => {
                *self = val.into();
                Ok(true)
            }
            None => Ok(false),
        }
    }
}
