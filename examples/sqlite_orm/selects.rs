use sequelles::Select;
use sequelles::SelectUnique;

use crate::structs::Pie;
use crate::structs::PieFilter;
use crate::structs::PiePrimaryKey;
use crate::structs::PieTopping;
use crate::structs::PieToppingPrimaryKey;
use crate::structs::Topping;
use crate::structs::ToppingName;

pub async fn selects(conn: &mut sqlx::SqliteConnection) {
    // Selecting a row is simple
    // Each unique key (Including the PK) has its own "[Table][Key]" struct. this allows selecting the proper key based on the type
    // You can also create the Key struct by using Key::from, and giving a tupple with each values in order of definition

    // Pk
    let _apple_pie = Pie::select_by_key(&mut *conn, PiePrimaryKey::from(1))
        .await
        .unwrap()
        .unwrap();

    let _apple_pie_topping = PieTopping::select_by_key(
        &mut *conn,
        PieToppingPrimaryKey {
            pie_id: 1,
            topping_id: 1,
        },
    )
    .await
    .unwrap()
    .unwrap();

    // By unique relation

    let name_filter = ToppingName::from("Apple".to_string());
    let _ = Topping::select_by_key(&mut *conn, name_filter)
        .await
        .unwrap()
        .unwrap();

    // Alternatively, you can filter on any field using the filter struct.
    // Please note that this uses more ressources at runtime than a proper sql query,
    // so if you are hungry for perfomance, avoid using it and make an handwritten query instead

    let filter = PieFilter::builder().price(5.25).build();
    let _ = Pie::select(conn, filter).await.unwrap().pop().unwrap();
}
