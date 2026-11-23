use itertools::Itertools as _;
use proc_macro2::Ident;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;

use crate::models::database_data::StructData;

pub fn create_filter_struct(data: &StructData) -> TokenStream {
    // Struct
    let filter_struct_name = Ident::new(&format!("{}Filter", data.struct_name), Span::call_site());

    let struct_fields = data.struct_fields
        .iter()
        .map(|f| {
            let name = Ident::new(&f.ident.as_ref().unwrap().to_string(), Span::call_site());
            let typ = &f.ty.clone();

            quote! {
                #[builder(into)]
                pub #name: Option<#typ>
            }
        })
        .collect_vec();

    quote! {
        #[derive(sequelles::bon::Builder)]
        pub struct #filter_struct_name {
            #(#struct_fields),*
        }
    }
}

