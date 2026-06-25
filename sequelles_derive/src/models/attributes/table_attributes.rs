use syn::Ident;

#[derive(deluxe::ExtractAttributes, Debug)]
#[deluxe(attributes(sequelles))]
pub struct TableStructAtribute {
    // Dialect attrs
    #[deluxe(default)]
    pub postgres: bool,
    #[deluxe(default)]
    pub sqlite: bool,

    // What to gen
    #[deluxe(default)]
    pub delete: bool,
    #[deluxe(default)]
    pub update: bool,
    #[deluxe(default)]
    pub insert: bool,
    #[deluxe(default)]
    pub select: bool,
    #[deluxe(default)]
    pub select_unique: bool,
    #[deluxe(default)]
    pub insert_struct: bool,
    #[deluxe(default)]
    pub upsert: bool,

    #[deluxe(flatten)]
    pub table_data: TableAtribute,
}

#[derive(deluxe::ParseMetaItem, Debug)]
pub struct TableAtribute {
    #[deluxe(default)]
    pub db_name: Option<String>,

    #[deluxe(default, append, alias = pk)]
    pub primary_key: Vec<TableUniqueKey>,

    #[deluxe(default, append, alias = unique, alias = unique_key)]
    pub unique_keys: Vec<TableUniqueKey>,

    #[deluxe(default, append, rename = foreign_key)]
    pub foreign_keys: Vec<TableForeignKey>,
}

#[derive(deluxe::ParseMetaItem, Debug, Clone)]
pub struct TableUniqueKey {
    #[deluxe(append)]
    pub columns: Vec<Ident>,

    pub key_name: String,
}

#[derive(deluxe::ParseMetaItem, Debug)]
pub struct TableForeignKey {
    #[deluxe(append)]
    columns: Vec<Ident>,

    references: String,

    #[deluxe(append)]
    ref_columns: Vec<String>,
}
