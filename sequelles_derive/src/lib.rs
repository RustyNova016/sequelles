use crate::delete::impl_delete_macro;
use crate::insert::impl_insert_macro;
use crate::select::impl_select_macro;
use crate::tables::impl_table_macro;
use crate::update::impl_update_macro;

mod delete;
mod insert;
mod macro_utils;
mod select;
mod tables;
mod update;

#[proc_macro_derive(Table, attributes(sequelles))]
pub fn table_macro_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let res = impl_table_macro(input.into());
    proc_macro::TokenStream::from(res)
}

#[proc_macro_derive(Update, attributes(sequelles))]
pub fn update_macro_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let res = impl_update_macro(input.into());
    proc_macro::TokenStream::from(res)
}

#[proc_macro_derive(Delete, attributes(sequelles))]
pub fn delete_macro_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let res = impl_delete_macro(input.into());
    proc_macro::TokenStream::from(res)
}

#[proc_macro_derive(Select, attributes(sequelles))]
pub fn select_macro_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let res = impl_select_macro(input.into());
    proc_macro::TokenStream::from(res)
}

#[proc_macro_derive(Insert, attributes(sequelles))]
pub fn insert_macro_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let res = impl_insert_macro(input.into());
    proc_macro::TokenStream::from(res)
}
