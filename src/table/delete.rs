pub trait Delete<C> {
    type Error;

    /// Delete the row in the database.
    fn delete(&self, conn: C) -> impl Future<Output = Result<(), Self::Error>>;
}


// impl<'c, T> Delete<sqlx::SqlitePool> for T
// where
//     T: for<'a> Delete<&'a mut sqlx::SqliteConnection>,
// {
//     type Error = <T as Delete<&'c mut sqlx::SqliteConnection>>::Error;

//     async fn delete(&self, conn: sqlx::SqlitePool) -> Result<(), Self::Error> {
//         let mut conn = conn.acquire().await.unwrap();
//         self.delete(&mut *conn).await
//     }
// }

