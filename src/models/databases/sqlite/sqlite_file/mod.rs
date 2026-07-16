use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

use snafu::ResultExt;
use sqlx::Connection;
use sqlx::SqliteConnection;
use sqlx::migrate::Migrate as _;
use sqlx::migrate::Migrator;
use sqlx::sqlite::SqliteConnectOptions;

use crate::models::databases::sqlite::sqlite_file::error::ConnectionSnafu;
use crate::models::databases::sqlite::sqlite_file::error::DatabaseArchivingSnafu;
use crate::models::databases::sqlite::sqlite_file::error::MigrationSnafu;
use crate::models::databases::sqlite::sqlite_file::error::SqliteFileError;

pub mod error;
#[cfg(test)]
pub mod tests;

/// Represent an SQLite file on disk
pub struct SqliteFile {
    path: PathBuf,
}

impl SqliteFile {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    /// Get the connection options out of the file
    pub fn as_connect_option(&self) -> SqliteConnectOptions {
        SqliteConnectOptions::new().filename(&self.path)
    }

    /// Migrate the current file and archive the old version
    pub async fn migrate_with_archive(
        &self,
        migrations: &Migrator,
        archive_folder_name: impl AsRef<Path>,
    ) -> Result<(), SqliteFileError> {
        // Connect to the DB, check the latest migration
        let conn_opt = SqliteConnectOptions::new()
            .create_if_missing(true)
            .filename(&self.path);
        let mut conn = SqliteConnection::connect_with(&conn_opt)
            .await
            .context(ConnectionSnafu)?;

        conn.ensure_migrations_table(&migrations.table_name)
            .await
            .context(MigrationSnafu)?;

        let appl_migrations = conn
            .list_applied_migrations(&migrations.table_name)
            .await
            .context(MigrationSnafu)?;

        let latest_migration = appl_migrations
            .iter()
            .map(|m| m.version)
            .max()
            .unwrap_or_default();

        // Is there migrations to do?
        if latest_migration
            >= migrations
                .iter()
                .map(|m| m.version)
                .max()
                .unwrap_or_default()
        {
            return Ok(());
        }

        // Migrations! We close the connection to remove the wal files.
        conn.close().await.context(ConnectionSnafu)?;

        let archive_folder = self
            .path
            .parent()
            .unwrap()
            .join(archive_folder_name)
            .join(latest_migration.to_string());
        fs::create_dir_all(&archive_folder).context(DatabaseArchivingSnafu)?;

        fs::copy(
            &self.path,
            archive_folder.join(self.path.file_name().unwrap_or(OsStr::new("database.db"))),
        )
        .context(DatabaseArchivingSnafu)?;

        let mut conn = SqliteConnection::connect(&self.path.display().to_string())
            .await
            .context(ConnectionSnafu)?;

        migrations.run(&mut conn).await.context(MigrationSnafu)?;

        Ok(())
    }
}
