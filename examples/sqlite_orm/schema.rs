
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

            -- Pie toppings
            CREATE TABLE `toppings` (
                `id` INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                `name` TEXT NOT NULL UNIQUE
            );

            -- What toppings the pie has
            CREATE TABLE `pie_toppings` (
                `pie_id` INTEGER NOT NULL REFERENCES `pies`(`id`),
                `topping_id` INTEGER NOT NULL UNIQUE REFERENCES `toppings`(`id`)
            );

            -- Record the date a pie got baked
            CREATE TABLE `baked_pies` (
                `pie_id` INTEGER NOT NULL,
                `baking_date` DATE NOT NULL,

                PRIMARY KEY (`pie_id`, `baking_date`)
            );

            -- Holds the shipment data
            CREATE TABLE `shipment` (
                `id` INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL
            );

            -- Associate the baked_pies to their shipments
            CREATE TABLE `shipped_pies` (
                `pie_id` INTEGER NOT NULL,
                `baking_date` DATE NOT NULL,
                `shipment_id` INTEGER NOT NULL REFERENCES `shipment` (`id`),

                PRIMARY KEY (`pie_id`, `baking_date`, `shipment_id`),
                FOREIGN KEY (`pie_id`, `baking_date`) REFERENCES `baked_pies` (`pie_id`, `baking_date`)
            );
        ",
    )
    .execute(conn)
    .await
    .unwrap();
}
