use sequelles_derive::HelloMacro;
use sqlx::prelude::FromRow;

#[derive(Debug, FromRow, HelloMacro)]
struct Pie {
    #[sequelles(pk, auto_increment)]
    pub id: i64,
    pub name: String,

    #[sqlx(rename = "sell_price")]
    #[sequelles(db_name = "sell_price")]
    pub price: f64,

    #[sequelles(unique)]
    pub barcode: String,
}

#[cfg(test)]
pub mod tests {
    use sqlx::Connection;
    use sqlx::SqliteConnection;

    use crate::SelectUnique;
    use crate::derive::Pie;
    use crate::derive::PieBarcode;
    use crate::derive::PiePk;

    #[tokio::test]
    pub async fn test_structs() {
        let mut conn = SqliteConnection::connect("sqlite::memory:").await.unwrap();

        sqlx::query(
            "
            CREATE TABLE `Pie` (
                `id` INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                `name` TEXT NOT NULL,
                `sell_price` NUMBER NOT NULL,
                `barcode` TEXT NOT NULL UNIQUE
            );
        ",
        )
        .execute(&mut conn)
        .await
        .unwrap();

        let apple_pie = Pie {
            id: 0,
            barcode: "123".to_string(),
            name: "Apple pie".to_string(),
            price: 2.25,
        }
        .insert(&mut conn)
        .await
        .unwrap();
        assert_eq!(apple_pie.id, 1);

        let mut apple_pie = Pie::select_by_pk(&mut conn, 1).await.unwrap().unwrap();
        assert_eq!(&apple_pie.name, "Apple");

        apple_pie.price = 1.95;
        apple_pie.update(&mut conn).await.unwrap();

        let mut apple_pie = Pie::select_by_pk(&mut conn, 1).await.unwrap().unwrap();
        assert_eq!(apple_pie.price, 1.95);

        let barcode = PieBarcode::from("123".to_string());
        let apple_pie = Pie::select_unique(&mut conn, barcode).await.unwrap();
    }
}
