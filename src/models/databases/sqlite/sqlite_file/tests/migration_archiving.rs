use core::str::FromStr;
use std::borrow::Cow;
use std::path::PathBuf;

use filium::models::directory::DirectoryPath;
use macro_rules_attribute::apply;
use smol_macros::test;
use sqlx::SqlStr;
use sqlx::migrate::Migration;
use sqlx::migrate::MigrationType;
use sqlx::migrate::Migrator;

use crate::models::databases::sqlite::sqlite_file::SqliteFile;

// Using macro attribute to go arround rust analyser freaking out everytime a character is inserted
#[apply(test!)]
async fn migration_archiving() {
    let test_folder = PathBuf::from("./target/tests/migration_archiving/");
    let test_dir = DirectoryPath::new(test_folder.clone());
    test_dir.remove_dir_all_or_ignore().unwrap();
    test_dir.create_dir_all().unwrap();

    let file = SqliteFile::new(
        PathBuf::from_str("./target/tests/migration_archiving/database.db").unwrap(),
    );

    let migr = Migration::new(
        1,
        Cow::Borrowed("Test"),
        MigrationType::Simple,
        SqlStr::from_static("CREATE TABLE test (id integer);"),
        false,
    );
    let migrator = Migrator::with_migrations(vec![migr]);

    file.migrate_with_archive(&migrator, ".db_archives")
        .await
        .unwrap();

    let archived = test_folder
        .as_path()
        .join(".db_archives")
        .join("0")
        .join("database.db");
    assert!(archived.exists());

    test_dir.remove_dir_all_or_ignore().unwrap();
}
