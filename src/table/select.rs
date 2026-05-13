pub trait SelectUnique<C, F>
where
    Self: Sized,
{
    /// Select a row by an unique identifier
    fn select_unique(
        conn: C,
        filter: F,
    ) -> impl Future<Output = Result<Option<Self>, sqlx::Error>>;
}

pub trait Select<C, F>
where
    Self: Sized,
{
    /// Select a row with a filter
    fn select(
        conn: C,
        filter: F,
    ) -> impl Future<Output = Result<Vec<Self>, sqlx::Error>>;
}
