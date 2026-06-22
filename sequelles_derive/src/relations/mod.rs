use proc_macro2::TokenStream;



pub mod impl_one_to_many;

// pub fn impl_relations_macro(item: TokenStream) -> TokenStream {
//     // Parse into an AST
//     let mut input = syn::parse2::<syn::DeriveInput>(item).unwrap();

//     // Parse the table data
//     let table_data = TableData::parse(&mut input);

//     // Build the macro
//     //insert_macro_inner(&table_data)
// }
