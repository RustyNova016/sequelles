#[derive(deluxe::ExtractAttributes, Debug)]
#[deluxe(attributes(sequelles))]
pub struct TableAtribute {
    pub db_name: Option<String>,
}
