use std::path::PathBuf;
use std::sync::Arc;

use async_once_cell::OnceCell;
use bon::Builder;
use deadpool::managed::Object;
use deadpool::managed::PoolConfig;
use snafu::Backtrace;
use snafu::ResultExt;
use snafu::Snafu;
use sqlx::SqliteConnection;
use sqlx::migrate::Migrator;
use sqlx::sqlite::SqliteConnectOptions;

use crate::databases::sqlite::database::pool::PoolInitError;
use crate::databases::sqlite::pool::SqlitePool;
use crate::databases::sqlite::pool::SqlitePoolConnection;
use crate::databases::sqlite::pool::SqlitePoolError;

pub mod pool;

pub type ArcSqliteDatabase = Arc<SqliteDatabase>;

/// All in one Sqlite database handler. Support filesystem operations, connection pooling.
#[derive(Debug, Builder)]
pub struct SqliteDatabase {
    pub path: Option<PathBuf>,

    /// The configuration of the connection
    pub connection_config: SqliteConnectOptions,

    /// The configuration of the pool
    pub pool_config: Option<PoolConfig>,

    /// The migrations of the database. If provided, they will be automatically be done on pool creation
    migrations: Option<Migrator>,

    #[builder(skip)]
    pool: OnceCell<SqlitePool>,
}

impl SqliteDatabase {
    /// Get a connection toward the database. Once dropped, it will return to the pool.
    ///
    /// This initialize the pool if it isn't ready yet.
    pub async fn get_conn(&self) -> Result<SqlitePoolConnection, GetConnectionError> {
        self.get_pool_or_init()
            .await
            .context(PoolInitSnafu)?
            .get()
            .await
            .context(ConnectionSnafu)
    }

    /// Get a connection toward the database. Once dropped, it will **not** return to the pool. You can use this to prevent RAII guard lifetime problems.
    ///
    /// This initialize the pool if it isn't ready yet.
    pub async fn get_conn_owned(&self) -> Result<SqliteConnection, GetConnectionError> {
        self.get_conn().await.map(Object::take)
    }
}

#[derive(Debug, Snafu)]
pub enum GetConnectionError {
    #[snafu(display("Could not create the pool for the database"))]
    PoolInitError {
        #[snafu(backtrace)]
        source: PoolInitError,
    },

    #[snafu(display("Could not get a connection from the database"))]
    ConnectionError {
        backtrace: Backtrace,
        source: SqlitePoolError,
    },
}
