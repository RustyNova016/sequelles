pub trait Insert<C> {
    type Output;

    fn insert(&self, conn: C) -> impl Future<Output = Result<Self::Output, sqlx::Error>>;
}
