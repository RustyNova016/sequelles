use sequelles::Table;
use sequelles::sqlx::FromRow;

// Let's write our structs. Please note that you both need FromRow and Table.
// sequelles::FromRow is just a re-export from sqlx
#[derive(Debug, FromRow, Table)]
///
// Select what you want to create (Opt-in for performance reasons)
#[sequelles(delete, update, insert, select, insert_struct, select_unique, upsert)]
//
// Select the databases to support
#[sequelles(postgres, sqlite)]
//
// Rename to use `pies` instead of `Pie`.
// While not obligatory, it's good practice to make the row singular, and the table plural
#[sequelles(db_name = "pies")]
//
// Add the constraints
#[sequelles(primary_key(key_name = "pk", columns(id)))]
#[sequelles(unique(key_name = "unique_barcode", columns(barcode)))]
pub struct Pie {
    // Let's read the schema and translate it.
    // `id` is an integer -> i64. It is the primary key. It's an auto_increment value
    #[sequelles(auto_increment)]
    pub id: i64,

    // `name` is a TEXT -> String
    pub name: String,

    // `description` is a TEXT -> String. Since it's nullable, we use an Option<>
    pub description: Option<String>,

    // `sell_price` is a NUMBER -> f64. However we want to rename it.
    // It also has a default. so we add default.
    // Please note that sequelles doesn't care about what the default is.
    // Because that's the database's job to set it. Not us.
    #[sqlx(rename = "sell_price")]
    //#[sequelles(db_name = "sell_price", default)]
    pub price: f64,

    // The barcode is unique. So we mark it as such
    //#[sequelles(unique)]
    pub barcode: String,
}
// // Alternatively you can pick which derive macros to run.
// // The Table macro is just #[derive(Insert, Select, Update, Delete)]
// #[derive(Debug, FromRow, Update, Delete, Select, Insert)]
// #[sequelles(db_name = "toppings")]
// pub struct Topping {
//     // `pk` is a shorthand for `primary_key` if you're lazy
//     #[sequelles(pk, auto_increment)]
//     pub id: i64,

//     #[sequelles(unique)]
//     pub name: String,
// }

// #[derive(Debug, FromRow, Table)]
// #[sequelles(db_name = "pie_toppings")]
// // You can declare multi columns primary keys like so:
// #[sequelles(primary_key(pie_id, topping_id))] // Wow just like SQL!
// pub struct PieTopping {
//     #[sequelles(references = "pies", with_row = Pie)]
//     pub pie_id: i64,

//     // You can declare multiple field as PK for composite keys
//     #[sequelles(references = "toppings", with_row = Topping)]
//     pub topping_id: i64,
// }

// #[derive(Debug, FromRow, Table)]
// #[sequelles(db_name = "baked_pies")]
// pub struct BakedPies {
//     #[sequelles(pk, references = "pies", with_row = Pie)]
//     pub pie_id: i64,

//     // You can declare multiple field as PK for composite keys
//     #[sequelles(pk, references = "toppings", with_row = Topping)]
//     pub baking_date: i64,
// }

// #[derive(Debug, FromRow, Table)]
// #[sequelles(db_name = "shipments")]
// pub struct Shipment {
//     #[sequelles(pk, auto_increment)]
//     pub id: i64,
// }

// #[derive(Debug, FromRow, Table)]
// #[sequelles(db_name = "shipped_pies")]
// pub struct ShippedPies {
//     #[sequelles(pk, auto_increment)]
//     pub id: i64,
// }
