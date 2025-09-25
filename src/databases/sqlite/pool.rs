use deadpool::managed;
use deadpool::managed::Object;
use deadpool::managed::PoolError;
use sqlx::Connection as _;
use sqlx::SqliteConnection;
use sqlx::sqlite::SqliteConnectOptions;

/// A [deadpool] manager for an sqlite database
#[derive(Debug)]
pub struct SqlitePoolManager {
    config: SqliteConnectOptions,
}

impl SqlitePoolManager {
    pub fn new(config: SqliteConnectOptions) -> Self {
        Self { config }
    }

    pub fn create_pool(config: SqliteConnectOptions) -> SqlitePool {
        SqlitePool::builder(SqlitePoolManager { config })
            .build()
            .unwrap()
    }
}

impl managed::Manager for SqlitePoolManager {
    type Type = sqlx::SqliteConnection;
    type Error = sqlx::Error;

    async fn create(&self) -> Result<Self::Type, Self::Error> {
        SqliteConnection::connect_with(&self.config).await
    }

    async fn recycle(
        &self,
        conn: &mut Self::Type,
        _: &managed::Metrics,
    ) -> managed::RecycleResult<Self::Error> {
        Ok(conn.ping().await?)
    }
}

/// A [deadpool] of sqlite connections
pub type SqlitePool = managed::Pool<SqlitePoolManager>;

pub type SqlitePoolError = PoolError<sqlx::Error>;

pub type SqlitePoolConnection = Object<SqlitePoolManager>;

pub type SqlitePoolResult = Result<SqlitePoolConnection, SqlitePoolError>;
