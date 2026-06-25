use proc_macro2::TokenStream;
use quote::quote;
use sequelles_sql_gen::models::schema::unique_key::UniqueKey;

use crate::models::database_data::StructData;
use crate::tables::unique_key::value_struct::create_ukey_struct::CreateUKeyStruct;
use crate::tables::unique_key::value_struct::impl_get_key_function::ImplGetKeyFunction;

pub mod create_ukey_struct;
pub mod impl_get_key_function;
pub mod select_with_ukey;

pub fn create_value_struct(data: &StructData, ukey: &UniqueKey) -> TokenStream {
    let key_struct_name = CreateUKeyStruct::get_struct_ident(data, ukey);

    let struct_doc = format!(
        "Represent the unique index `{}` for the table {}",
        ukey.name, data.table.name
    );

    let ukey_fields = ukey.fields.iter().map(|db_field| {
        let field = data.get_field_by_name(&db_field.rust_name).unwrap();
        let name = &field.ident;
        let typ = &field.ty;
        quote! {pub #name: #typ}
    });

    let impl_get_key_func =
        ImplGetKeyFunction::gen_impl(data, &ukey, &key_struct_name, &data.struct_name);

    quote! {
        #[doc = #struct_doc]
        pub struct #key_struct_name {
            #(#ukey_fields),*
        }

        #impl_get_key_func
    }
}
