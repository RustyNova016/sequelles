use sequelles::SelectUnique;
use sequelles::Update;

use crate::structs::Pie;
use crate::structs::PieUniqueBarcode;

pub async fn updates(conn: &mut sqlx::SqliteConnection) {
    // Updating has only one way. Just call .update() on the row

    let mut apple_pie = Pie::select_unique(
        &mut *conn,
        PieUniqueBarcode {
            barcode: "MyBarcode".to_string(),
        },
    )
    .await
    .unwrap()
    .unwrap();

    apple_pie.price = 6.31;

    let new_apple_pie = apple_pie.update(conn).await.unwrap().unwrap();
    assert_eq!(new_apple_pie.price,  6.31);
}
