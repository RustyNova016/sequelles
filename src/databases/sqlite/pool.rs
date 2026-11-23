use deadpool::managed;
use deadpool::managed::Object;
use deadpool::managed::PoolError;
use deadpool::managed::RecycleError;
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
        // Is the connection alive?
        conn.ping().await?;

        // If in transaction, remove conn
        if conn.is_in_transaction() {
            return Err(RecycleError::message(
                "Connection in transaction. Couldn't recycle it",
            ));
        }

        Ok(())
    }
}

/// A [deadpool] of sqlite connections
pub type SqlitePool = managed::Pool<SqlitePoolManager>;

pub type SqlitePoolError = PoolError<sqlx::Error>;

pub type SqlitePoolConnection = Object<SqlitePoolManager>;

pub type SqlitePoolResult = Result<SqlitePoolConnection, SqlitePoolError>;
