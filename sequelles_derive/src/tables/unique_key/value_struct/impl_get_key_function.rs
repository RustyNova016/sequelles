use convert_case::Case;
use convert_case::Casing;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use sequelles_sql_gen::models::schema::unique_key::UniqueKey;
use syn::Ident;

use crate::models::database_data::StructData;

pub struct ImplGetKeyFunction;

impl ImplGetKeyFunction {
    fn function_name(ukey: &UniqueKey) -> Ident {
        Ident::new(
            &format!(
                "get_{}_unique_key",
                ukey.name.to_string().to_case(Case::Snake)
            ),
            Span::call_site(),
        )
    }

    pub fn gen_impl(data: &StructData, ukey: &UniqueKey, key_struct_name: &Ident, for_struct: &Ident) -> TokenStream {
        if !data.gen_unique_keys {
            return quote! {}
        }

        let function_name = Self::function_name(ukey);

        let ukey_fields = ukey.fields.iter().map(|db_field| {
            let field = data.get_field_by_name(&db_field.rust_name).unwrap();
            let name = &field.ident;
            quote! {#name: self.#name.clone()}
        });

        quote! {
            impl #for_struct {
                pub fn #function_name(&self) -> #key_struct_name {
                    #key_struct_name {
                        #(#ukey_fields),*
                    }
                }
            }
        }
    }
}


