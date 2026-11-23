use proc_macro2::TokenStream;
use quote::quote;
use syn::Field;
use syn::Ident;

use crate::tables::insert_struct::create_struct::CreateInsertStruct;

impl CreateInsertStruct {
    pub fn impl_from_row_struct(row_struct: &Ident, insert_struct: &Ident, row_fields: &[Field]) -> TokenStream {
        let fields = row_fields.iter().map(|field| {
            let name = &field.ident;
            quote! {#name: value.#name.into()}
        });

        quote! {
            impl From<#row_struct> for #insert_struct {
                fn from(value: #row_struct) -> #insert_struct {
                    #insert_struct {
                        #(#fields),*
                    }
                }
            }

            impl #row_struct {
                pub fn into_insertable(self) -> #insert_struct {
                    self.into()
                }
            }
        }
    }
}