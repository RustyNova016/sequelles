use proc_macro2::TokenStream;
use sequelles_sql_gen::models::schema::unique_key::UniqueKey;

use crate::models::database_data::StructData;
use crate::tables::insert_struct::create_struct::CreateInsertStruct;
use crate::tables::unique_key::value_struct::create_ukey_struct::CreateUKeyStruct;
use crate::tables::unique_key::value_struct::impl_get_key_function::ImplGetKeyFunction;

impl CreateInsertStruct {
    pub fn gen_get_key_funcs(data: &StructData) -> TokenStream {
        data.table
            .unique_keys()
            .iter()
            .filter(|ukey| !ukey.has_default_column())
            .map(|ukey| Self::gen_get_key_func(data, ukey))
            .collect()
    }

    fn gen_get_key_func(data: &StructData, ukey: &UniqueKey) -> TokenStream {
        let key_struct_name = CreateUKeyStruct::get_struct_ident(data, ukey);
        ImplGetKeyFunction::gen_impl(data, ukey, &key_struct_name, &Self::get_struct_ident(data))
    }
}
