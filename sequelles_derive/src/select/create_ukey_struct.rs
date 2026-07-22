use core::fmt::Display;

use convert_case::Case;
use convert_case::Casing as _;
use itertools::Itertools;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::macro_utils::table_definition::unique_key::UniqueKeyData;
use crate::select::impl_select_key_trait::impl_select_trait;
use crate::select::ukey_struct_from::ukey_from;

pub fn create_ukey_struct(
    impl_struct: &Ident,
    table_name: impl Display,
    unique_key: &UniqueKeyData,
) -> TokenStream {
    let struct_name = Ident::new(
        &format!(
            "{impl_struct}{}",
            unique_key.name.to_string().to_case(Case::Pascal)
        ),
        Span::call_site(),
    );
    let struct_doc = format!(
        "Represent the unique index `{}` for the table {table_name}",
        unique_key.name
    );

    let from = ukey_from(&unique_key.fields_as_ref_iter().collect_vec(), &struct_name);
    let select = impl_select_trait(
        impl_struct,
        &quote! {&mut sqlx::SqliteConnection},
        &struct_name,
        table_name,
        &unique_key,
    );

    let ukey_fields = unique_key.fields_as_ref_iter();

    quote! {
        #[doc = #struct_doc]
        pub struct #struct_name {
            #(#ukey_fields),*
        }

        #from

        #select
    }
}
