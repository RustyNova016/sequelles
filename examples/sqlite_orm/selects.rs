use sequelles::SelectUnique;

use crate::structs::Pie;
use crate::structs::PieTopping;
use crate::structs::Topping;
use crate::structs::ToppingName;
use crate::structs::ToppingPk;

pub async fn selects(conn: &mut sqlx::SqliteConnection) {
    // Selecting a row is simple

    // By PK
    let _apple_pie = Pie::select_by_pk(conn, 1).await.unwrap().unwrap();
    let _apple_pie_topping = PieTopping::select_by_pk(conn, 1, 1).await.unwrap().unwrap();

    // By unique relation
    // Each unique relation has its own "[Table]Key" struct. this allows selecting the proper key based on the type
    let name_filter = ToppingName::from("Apple".to_string());
    let _ = Topping::select_unique(&mut *conn, name_filter)
        .await
        .unwrap()
        .unwrap();

    // Also works for the PK!
    let pk_filter = ToppingPk::from(1);
    let _ = Topping::select_unique(&mut *conn, pk_filter)
        .await
        .unwrap()
        .unwrap();
}
