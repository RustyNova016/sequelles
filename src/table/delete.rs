use sqlx::SqliteConnection;
use sqlx::SqlitePool;

pub trait Delete<C> {
    /// Delete the row in the database.
    fn delete(&self, conn: C) -> impl Future<Output = Result<(), sqlx::Error>>;
}

impl<T> Delete<SqlitePool> for T
where
    T: for<'a> Delete<&'a mut SqliteConnection>,
{
    async fn delete(&self, conn: SqlitePool) -> Result<(), sqlx::Error> {
        let mut conn = conn.acquire().await?;
        self.delete(&mut *conn).await
    }
}
