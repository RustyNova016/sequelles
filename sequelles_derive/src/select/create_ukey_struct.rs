use core::fmt::Display;

use convert_case::Case;
use convert_case::Casing as _;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::select::impl_select_key_trait::impl_select_trait;
use crate::select::ukey_struct_from::ukey_from;
use crate::tables::table_data::field_data::FieldData;

pub fn create_ukey_struct(
    impl_struct: &Ident,
    table_name: impl Display,
    ukey_name: impl Display,
    ukey_fields: &[&FieldData],
) -> TokenStream {
    let struct_name = Ident::new(
        &format!(
            "{impl_struct}{}",
            ukey_name.to_string().to_case(Case::Pascal)
        ),
        Span::call_site(),
    );
    let struct_doc = format!("Represent the unique index `{ukey_name}` for the table {table_name}");

    let from = ukey_from(ukey_fields, &struct_name);
    let select = impl_select_trait(
        impl_struct,
        &quote! {&mut sqlx::SqliteConnection},
        &struct_name,
        table_name,
        ukey_fields,
    );

    quote! {
        #[doc = #struct_doc]
        pub struct #struct_name {
            #(#ukey_fields),*
        }

        #from

        #select
    }
}
