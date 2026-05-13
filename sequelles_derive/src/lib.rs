use crate::tables::impl_hello_macro;

mod tables;
mod macro_utils;

#[proc_macro_derive(Table, attributes(sequelles))]
pub fn table_macro_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let res = impl_hello_macro(input.into());
    proc_macro::TokenStream::from(res)
}

