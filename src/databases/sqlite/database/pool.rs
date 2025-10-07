use async_once_cell::OnceCell;
use snafu::Backtrace;
use snafu::ResultExt as _;
use snafu::Snafu;
use sqlx::SqliteConnection;
use sqlx::migrate::MigrateError;

use crate::databases::sqlite::database::SqliteDatabase;
use crate::databases::sqlite::pool::SqlitePool;
use crate::databases::sqlite::pool::SqlitePoolError;
use crate::databases::sqlite::pool::SqlitePoolManager;

impl SqliteDatabase {
    pub async fn init_pool<F>(&self, pool: F) -> Result<SqlitePool, PoolInitError>
    where
        F: FnOnce() -> SqlitePool,
        SqliteConnection:,
    {
        let pool = pool();

        if let Some(migrator) = self.migrations.as_ref() {
            let conn = &mut *pool.get().await.context(ConnectionSnafu)?;

            // Migrator is **not** send, which mess up this function's future to not be send, meaning that upstream uses cannot send anything
            // The block_on call restore this. This shouldn't really be an issue to not async it as DB initialisation isn't really paired with other
            // concurent actions.
            //
            // Thanks Sqlx spaguetti.
            //
            // See: https://github.com/launchbadge/sqlx/issues/954#issuecomment-767080149
            futures::executor::block_on(async { migrator.run(conn).await })
                .context(MigrationSnafu)?
        }

        Ok(pool)
    }

    /// Initialize the internal pool. Useful to pass in a custom pool
    ///
    /// Does nothing if the pool is already initialized
    pub async fn get_pool_or_init_with<F>(&self, pool: F) -> Result<&SqlitePool, PoolInitError>
    where
        F: FnOnce() -> SqlitePool,
    {
        self.pool.get_or_try_init(self.init_pool(pool)).await
    }

    /// Get the inner pool or initialize it and return it
    pub async fn get_pool_or_init(&self) -> Result<&SqlitePool, PoolInitError> {
        self.get_pool_or_init_with(move || {
            SqlitePool::builder(SqlitePoolManager::new(self.connection_config.to_owned()))
                .config(self.pool_config.to_owned().unwrap_or_default())
                .build()
                .expect("Couldn't build the sqlite pool")
        })
        .await
    }

    /// Close the connection pool by dropping it.
    ///
    /// It isn't closed forever, as it may be reopened at anytime be calling [SqliteDatabase::get_pool_or_init] or [SqliteDatabase::get_conn]
    pub fn close_pool(&mut self) {
        self.pool = OnceCell::new()
    }
}

#[derive(Debug, Snafu)]
pub enum PoolInitError {
    #[snafu(display("Could not get a connection from the database"))]
    ConnectionError {
        backtrace: Backtrace,
        source: SqlitePoolError,
    },

    #[snafu(display("Could not apply the migrations"))]
    MigrationError {
        backtrace: Backtrace,
        source: MigrateError,
    },
}
