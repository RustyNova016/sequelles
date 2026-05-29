use sqlx::Connection;
use sqlx::SqliteConnection;

use crate::inserts::inserts;
use crate::schema::generate_database;
use crate::selects::selects;

pub mod inserts;
pub mod schema;
pub mod selects;
pub mod structs;

#[tokio::main]
pub async fn main() {
    env_logger::init();
    let mut conn = SqliteConnection::connect("sqlite::memory:").await.unwrap();

    // To better follow this exemple, check the files in this order:
    // schema.rs
    generate_database(&mut conn).await;

    // structs.rs

    // inserts.rs
    inserts(&mut conn).await;

    // selects.rs
    selects(&mut conn).await;
}
