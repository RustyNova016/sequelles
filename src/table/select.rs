/// Trait to select a row by a filter that target a specific row.
pub trait SelectUnique<C, F>
where
    Self: Sized,
{
    /// Select a specific row with a filter.
    fn select_unique(conn: C, filter: F) -> impl Future<Output = Result<Option<Self>, sqlx::Error>>;
}

/// Trait to select rows that fit a specific filter.
///
/// If the filter always return a maximum of one row, please implement [`SelectOne`] instead
pub trait Select<C, F>
where
    Self: Sized,
{
    /// Select a row with a filter
    fn select(conn: C, filter: F) -> impl Future<Output = Result<Vec<Self>, sqlx::Error>>;
}
