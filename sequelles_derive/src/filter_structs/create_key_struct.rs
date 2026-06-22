use core::fmt::Display;

use convert_case::Case;
use convert_case::Casing;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::ToTokens;
use quote::quote;
use syn::Ident;

use crate::macro_utils::table_definition::field_data::FieldData;

/// Create a key struct. This is a struct that create a verbatim filter
pub fn create_key_struct(name: &str, docs: impl ToTokens, fields: &[&FieldData]) -> TokenStream {
    let struct_name = Ident::new(&name.to_case(Case::Pascal), Span::call_site());

    quote! {
        #[doc = #docs]
        pub struct #struct_name {
            #(#fields),*
        }
    }
}
