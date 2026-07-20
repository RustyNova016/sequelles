use sequelles::Select;
use sequelles::SelectUnique;

use crate::structs::Pie;
use crate::structs::PieFilter;
use crate::structs::PieTopping;
use crate::structs::PieToppingPk;
use crate::structs::PieUniqueBarcode;

pub async fn selects(conn: &mut sqlx::SqliteConnection) {
    // There is 2 ways to select data

    // -------------------------------------------------------------------------------------------------------------------------------------------
    //
    // Unique rows (#[sequelles(select_unique)])

    // Each unique key (Including the PK) has its own "{Row}{Key}" struct. This allows filtering to get a specific row.

    // Pk
    let _apple_pie_topping = PieTopping::select_unique(
        &mut *conn,
        PieToppingPk {
            pie_id: 1,
            topping_id: 1,
        },
    )
    .await
    .unwrap()
    .unwrap();

    // Unique
    let _apple_pie = Pie::select_unique(
        &mut *conn,
        PieUniqueBarcode {
            barcode: "MyBarcode".to_string(),
        },
    )
    .await
    .unwrap()
    .unwrap();

    // -------------------------------------------------------------------------------------------------------------------------------------------
    //
    // Multiple rows (#[sequelles(select)])
    //
    // This generate a {Row}Filter struct that can be used to set filters
    //
    // Please note that this uses more ressources at runtime than a proper sql query,
    // so if you are hungry for perfomance, avoid using it and make an handwritten query instead

    let filter = PieFilter::builder().price(5.25).build();
    let _ = Pie::select(conn, filter).await.unwrap().pop().unwrap();
}
