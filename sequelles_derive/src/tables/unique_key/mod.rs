use proc_macro2::TokenStream;
use quote::quote;

use crate::models::database_data::StructData;
use crate::tables::unique_key::value_struct::create_value_struct;
use crate::tables::unique_key::value_struct::select_with_ukey::impl_select_unique_trait;

pub mod value_struct;

pub fn impl_unique_structs(data: &StructData) -> TokenStream {
    if !data.gen_unique_keys {
        return quote! {};
    }

    data.table.iter_keys().map(|ukey| {
        let key_struct = create_value_struct(data, ukey);
        let selects = impl_select_unique_trait(data, ukey);

        quote! {
            #key_struct
            #selects
        }
    }).collect()
}
