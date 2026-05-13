/// Let's create the database. This is standard sqlx, altough it would be better to use a migration.
pub async fn generate_database(conn: &mut sqlx::SqliteConnection) {
    sqlx::query(
        "
            CREATE TABLE `pies` (
                `id` INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                `name` TEXT NOT NULL,
                `description` TEXT,
                `sell_price` NUMBER DEFAULT 2.99,
                `barcode` TEXT NOT NULL UNIQUE
            );

            CREATE TABLE `toppings` (
                `id` INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                `name` TEXT NOT NULL UNIQUE
            );

            CREATE TABLE `pie_toppings` (
                `pie_id` INTEGER NOT NULL REFERENCES `pies`(`id`),
                `topping_id` INTEGER NOT NULL UNIQUE  REFERENCES `toppings`(`id`)
            );
        ",
    )
    .execute(conn)
    .await
    .unwrap();
}
