pub trait Upsert<C> {
    type Output;
    type Error;

    /// Insert the row into the database. If there's any constraint preventing the insert, ignore and returns none
    fn upsert(&self, conn: C) -> impl Future<Output = Result<Option<Self::Output>, Self::Error>>;
}

pub trait UpsertSelf<C> {
    type Error;

    /// Insert the row into the database, and save the result into self. If there's any constraint preventing the insert, ignore and returns false
    fn upsert_self(&mut self, conn: C) -> impl Future<Output = Result<bool, Self::Error>>;
}

impl<C, T> UpsertSelf<C> for T
where
    T: Upsert<C>,
    T::Output: Into<T>,
{
    type Error = T::Error;
    
    async fn upsert_self(&mut self, conn: C) -> Result<bool, T::Error> {
        match self.upsert(conn).await? {
            Some(val) => {
                *self = val.into();
                Ok(true)
            }
            None => Ok(false),
        }
    }
}

// #[cfg(feature = "sqlite")]
// impl<T> Upsert<sqlx::SqlitePool> for T
// where
//     T: for<'a> Upsert<&'a mut sqlx::SqliteConnection>,
// {
//     type Output = <T as Upsert<&mut SqliteConnection>>::Output;

//     async fn upsert(&self, conn: sqlx::SqlitePool) -> Result<Option<Self::Output>, sqlx::Error> {
//         let mut conn = conn.acquire().await?;
//         self.upsert(&mut *conn).await
//     }
// }

// #[cfg(feature = "postgres")]
// impl<T> Upsert<sqlx::PgPool> for T
// where
//     T: for<'a> Upsert<&'a mut sqlx::PgConnection>,
// {
//     type Output = T::Output;
//     async fn upsert(&self, conn: sqlx::PgPool) -> Result<(), sqlx::Error> {
//         let mut conn = conn.acquire().await?;
//         self.upsert(&mut *conn).await
//     }
// }
