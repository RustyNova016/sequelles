use syn::Ident;

#[derive(deluxe::ExtractAttributes, Debug)]
#[deluxe(attributes(sequelles))]
pub struct ColumnAtribute {
    // #[deluxe(default, alias = pk)]
    // pub primary_key: bool,

    #[deluxe(default)]
    pub auto_increment: bool,

    // #[deluxe(default)]
    // pub unique: bool,

    #[deluxe(default)]
    pub default: bool,

    pub db_name: Option<String>,

    // #[deluxe(flatten)]
    // pub relation: RelationAtribute,
}

#[derive(deluxe::ParseMetaItem, Debug)]
pub struct RelationAtribute {
    pub references: Option<String>,
    pub with_struct: Option<Ident>,
}
