pub mod databases;
pub mod datastructures;
pub mod models;
pub mod table;
pub mod tables;
//mod derive;

pub use crate::datastructures::rowid_map::RowIDMap;
pub use crate::tables::table::Table;
pub use crate::tables::traits::has_rowid;

pub use crate::datastructures::joins::*;

pub use crate::table::delete::Delete;
pub use crate::table::inserts::InsertOrIgnore;
pub use crate::table::inserts::InsertOrIgnoreSelf;
pub use crate::table::relation::ManyToOne;
pub use crate::table::relation::OneToMany;
pub use crate::table::select::Select;
pub use crate::table::select::SelectUnique;
pub use crate::table::update::Update;

// pub use sequelles_derive::Delete;
// pub use sequelles_derive::Insert;
// pub use sequelles_derive::Select;
pub use sequelles_derive::Table;
// pub use sequelles_derive::Update;

pub mod bon {
    pub use bon::*;
}

pub mod sea_query {
    pub use sea_query::*;
}

pub mod sea_query_sqlx {
    pub use sea_query_sqlx::*;
}

pub mod sqlx {
    pub use sqlx::*;
}
