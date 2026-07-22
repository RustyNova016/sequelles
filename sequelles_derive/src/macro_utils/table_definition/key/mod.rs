pub mod foreign_key;
use core::fmt::Display;
use std::rc::Rc;

use convert_case::Case;
use convert_case::Casing;
use proc_macro2::Ident;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;

use crate::macro_utils::table_definition::field_data::FieldData;
use crate::macro_utils::table_definition::sql_dialect::Sqlite;
use crate::macro_utils::trait_impl_gen::select_key::SelectKeyImpl;
use crate::macro_utils::trait_impl_gen::select_one::SelectOneImpl;

pub struct TableKey {
    pub name: String,
    pub description: String,

    pub fields: Vec<Rc<FieldData>>,
    pub unique: bool,
}

impl TableKey {
    pub fn generate_struct(&self) -> TokenStream {
        let struct_name = self.struct_name();
        let docs = &self.description;
        let fields = &self.fields;

        quote! {
            #[doc = #docs]
            pub struct #struct_name {
                #(#fields),*
            }
        }
    }

    fn struct_name(&self) -> Ident {
        Ident::new(&self.name.to_case(Case::Pascal), Span::call_site())
    }

    pub fn generate_select_one(&self, for_struct: &Ident, table_name: impl Display) -> TokenStream {
        SelectOneImpl::<Sqlite>::generate(table_name, &self, for_struct, &self.struct_name())
    }

    pub fn generate_select_key(&self, for_struct: &Ident, table_name: impl Display) -> TokenStream {
        SelectKeyImpl::<Sqlite>::generate(table_name, &self, for_struct, &self.struct_name())
    }
}
