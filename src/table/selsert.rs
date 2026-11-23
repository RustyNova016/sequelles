pub trait Selsert<C> {
    type Output;
    type Error;

    /// Select by an unique key, and if it doesn't exists, insert it
    fn selsert(&self, conn: C)-> impl Future<Output = Result<Self::Output, Self::Error>>;
}
