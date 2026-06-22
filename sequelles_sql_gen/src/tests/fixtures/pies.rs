use std::rc::Rc;

use crate::models::schema::field::Column;
use crate::models::schema::table::Table;
use crate::models::schema::unique_key::UniqueKey;

pub fn fixture_pies() -> Table {
    let mut table = Table::new("pies".to_string(), None);

    let id_field = Rc::new(Column {
        sql_name: "id".to_string(),
        rust_name: "id".to_string(),
        auto_increment: true,
        default: false,
        //not_null: true,
    });
    table.fields.push(&id_field);

    table.fields.push(&Rc::new(Column {
        sql_name: "name".to_string(),
        rust_name: "name".to_string(),
        auto_increment: true,
        default: false,
        //not_null: true,
    }));

    let mut pkey = UniqueKey::new("primary_key".to_string(), true);
    pkey.fields.push(&id_field);
    table.add_unique_key(pkey);

    table
}
