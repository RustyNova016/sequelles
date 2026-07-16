use std::io;

use snafu::Location;
use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(super)))]
pub enum SqliteFileError {
    #[snafu(display("An error happened while connecting to the database"))]
    ConnectionError {
        source: sqlx::Error,
        #[snafu(implicit)]
        location: Location,
    },

    #[snafu(display("An error happened while applying the migrations"))]
    MigrationError {
        source: sqlx::migrate::MigrateError,
        #[snafu(implicit)]
        location: Location,
    },

    #[snafu(display("An error happened while archiving the old database"))]
    DatabaseArchivingError {
        source: io::Error,
        #[snafu(implicit)]
        location: Location,
    },
}
