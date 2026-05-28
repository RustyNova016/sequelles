pub trait SelectKey<C, F>
where
    Self: Sized,
{
    /// Select a row by a key
    fn select_by_key(conn: C, filter: F)
    -> impl Future<Output = Result<Option<Self>, sqlx::Error>>;
}

pub trait Select<C, F>
where
    Self: Sized,
{
    /// Select a row with a filter
    fn select(conn: C, filter: F) -> impl Future<Output = Result<Vec<Self>, sqlx::Error>>;
}
