use sequelles::InsertOrIgnore;
use sequelles::InsertedValue;

use crate::structs::PieInsertableDyn;
use crate::structs::PieTopping;
use crate::structs::ToppingInsert;

pub async fn inserts(conn: &mut sqlx::SqliteConnection) {
    // There's 3 ways to insert a new row.

    // -------------------------------------------------------------------------------------------------------------------------------------------
    //
    // 1 - #[sequelles(insert_struct)]
    // This create a {RowStruct}Insert struct. This has the same structure as the row struct, but all the default values are set as options:

    // You can create the struct by destructuring...
    let _apple_topping = ToppingInsert {
        id: None,
        name: "Apple".to_string(),
    };

    // ... But you may also prefer using a builder pattern
    let apple_topping = ToppingInsert::builder().name("Apple").build();

    // Then call .insert_or_ignore(). This will only insert the row if no constraints are blocking it.
    // If the insert is successful, returns the row

    let topping_id = apple_topping
        .insert_or_ignore(&mut *conn)
        .await
        .unwrap()
        .unwrap()
        .id;

    // -------------------------------------------------------------------------------------------------------------------------------------------
    //
    // 2 - #[sequelles(dyn_insert_struct)]
    // This create a {RowStruct}InsertableDyn struct.
    // The difference between this and the {RowStruct}Insert struct is that it builds a insert statement at run time instead of compile time,
    // And require a [InsertedValue] for values with defaults.
    //
    // This is useful for SQL dialects that, like PostgeSQL, do not convert `NULL` into `DEFAULT`.

    let apple_pie = PieInsertableDyn {
        name: "Apple Pie".to_string(),
        barcode: "MyBarcode".into(),
        description: None,
        id: InsertedValue::Default,
        price: InsertedValue::Some(5.25),
    };

    let pie_id = apple_pie.insert_or_ignore(&mut *conn).await.unwrap().unwrap().id;

    // -------------------------------------------------------------------------------------------------------------------------------------------
    //
    // 3 - #[sequelles(insert)]
    // This add a .insert() method directly on the row struct
    // It's the most restricted method of insertion as all fields must be filled. This shines on rows that have no optional nor default values

    PieTopping { pie_id, topping_id }
        .insert_or_ignore(&mut *conn)
        .await
        .unwrap()
        .unwrap();

    // Keep in mind that you want to keep the result of the insert, not the row.
    // This is simply because the database may edit the inserted row in triggers, or set defaults
    // Also, if insertion fails, it will return a None
}
