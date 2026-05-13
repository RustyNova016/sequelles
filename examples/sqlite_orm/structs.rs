use sequelles::FromRow;
use sequelles::Table;
use sqlx::Sqlite;

// Let's write our structs. Please note that you both need FromRow and Table.
// sequelles::FromRow is just a re-export from sqlx ;)
#[derive(Debug, FromRow, Table)]
// Rename to use `pies` instead of `Pie`.
// While not obligatory, it's good practice to make the row singular, and the table plural
#[sequelles(db_name = "pies")]
pub struct Pie {
    // Let's read the schema and translate it.
    // `id` is an integer -> i64. It is the primary key. It's an auto_increment value
    #[sequelles(pk, auto_increment)]
    pub id: i64,

    // `name` is a TEXT -> String
    pub name: String,

    // `description` is a TEXT -> String. Since it's nullable, we use an Option<>
    pub description: Option<String>,

    // `id` is a NUMBER -> f64. However we want to rename it.
    // It also has a default. so we add default.
    // Please note that sequelles doesn't care about what the default is.
    // Because that's the database's job to set it. Not us.
    #[sqlx(rename = "sell_price")]
    #[sequelles(db_name = "sell_price", default)]
    pub price: f64,

    // The barcode is unique. So we mark it as such
    #[sequelles(unique)]
    pub barcode: String,
}

// Now for the other structs

#[derive(Debug, FromRow, Table)]
#[sequelles(db_name = "toppings")]
pub struct Topping {
    #[sequelles(pk, auto_increment)]
    pub id: i64,

    #[sequelles(unique)]
    pub name: String,
}

#[derive(Debug, FromRow, Table)]
#[sequelles(db_name = "pie_toppings")]
pub struct PieTopping {
    #[sequelles(pk, references = "pies", with_row = Pie)]
    pub pie_id: i64,

    // You can declare multiple field as PK for composite keys
    #[sequelles(pk, references = "toppings", with_row = Topping)]
    pub topping_id: i64,
}
