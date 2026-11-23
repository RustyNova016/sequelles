pub trait OneToMany<Relation, C> {
    type Other;

    /// Fetch related many rows
    fn get_many(
        &self,
        rel: Relation,
        conn: C,
    ) -> impl std::future::Future<Output = Self::Other> + Send;
}

pub trait ManyToOne<Relation, C> {
    type Other;

    /// Fetch related one row
    fn get_one(
        &self,
        rel: Relation,
        conn: C,
    ) -> impl std::future::Future<Output = Self::Other> + Send;
}
