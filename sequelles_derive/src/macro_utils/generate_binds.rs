use proc_macro2::Ident;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use sequelles_sql_gen::models::sql_bind::SqlBinds;

pub fn generate_self_binds(binds: &SqlBinds) -> TokenStream {
    generate_binds(&quote! {self}, binds)
}

pub fn generate_binds(var: &TokenStream, binds: &SqlBinds) -> TokenStream {
    let mut res = quote! {};

    for (_, field) in binds.iter() {
        let field = Ident::new(field, Span::call_site());
        res = quote! {
            #res.bind(&#var.#field)
        }
    }

    res
}
