use convert_case::Case;
use convert_case::Casing;
use proc_macro2::Span;
use sequelles_sql_gen::models::schema::unique_key::UniqueKey;
use syn::Ident;

use crate::models::database_data::StructData;

pub struct CreateUKeyStruct;

impl CreateUKeyStruct {
    pub fn get_struct_ident(data: &StructData, ukey: &UniqueKey) -> Ident {
        Ident::new(
            &format!(
                "{}{}",
                data.struct_name,
                ukey.name.to_string().to_case(Case::Pascal)
            ),
            Span::call_site(),
        )
    }
}
