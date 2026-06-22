pub mod select_with_ukey;
use convert_case::Case;
use convert_case::Casing;
use proc_macro2::Ident;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use sequelles_sql_gen::models::schema::unique_key::UniqueKey;

use crate::models::database_data::StructData;

pub fn create_value_struct(data: &StructData, ukey: &UniqueKey) -> TokenStream {
    let struct_name = Ident::new(
        &format!(
            "{}{}",
            data.struct_name,
            ukey.name.to_string().to_case(Case::Pascal)
        ),
        Span::call_site(),
    );

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

    quote! {
        #[doc = #struct_doc]
        pub struct #struct_name {
            #(#ukey_fields),*
        }
    }
}
