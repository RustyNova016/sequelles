use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::models::database_data::StructData;

pub fn create_error_enum(data: &StructData) -> TokenStream {
    if !data.gen_snafu {
        return quote! {};
    }

    let mut inner_variants = quote! {};

    if data.gen_delete {
        inner_variants.extend(error_variant(
            data,
            "Delete",
            "An error happened while deleting a row",
        ));
    }

    if data.gen_insert || data.gen_insert_struct {
        inner_variants.extend(error_variant(
            data,
            "Insert",
            "An error happened while inserting a row",
        ));
    }

    if data.gen_update {
        inner_variants.extend(error_variant(
            data,
            "Update",
            "An error happened while updating a row",
        ));
    }

    if data.gen_select {
        inner_variants.extend(error_variant(
            data,
            "Select",
            "An error happened while querying a row",
        ));
    }

    if data.gen_upsert {
        inner_variants.extend(error_variant(
            data,
            "Upsert",
            "An error happened while upserting a row",
        ));
    }

    if data.gen_select_unique {
        inner_variants.extend(error_variant(
            data,
            "SelectUnique",
            "An error happened while querying an unique row",
        ));
    }

    if data.gen_selsert {
        inner_variants.extend(error_variant(
            data,
            "SelsertTransaction",
            "An error happened while selserting row",
        ));

        inner_variants.extend(option_error_variant(
            data,
            "SelsertNotFound",
            "Couldn't find the row after inserting it.",
        ));
    }

    let name = Ident::new(&format!("{}SqlError", data.struct_name), Span::call_site());

    quote! {
        #[derive(Debug, sequelles::snafu::Snafu)]
        pub enum #name {
            #inner_variants
        }
    }
}

fn error_variant(data: &StructData, action: &str, doc: &str) -> TokenStream {
    let name = Ident::new(
        &format!("{}{action}Error", data.struct_name),
        Span::call_site(),
    );

    quote! {
        #[snafu(display(#doc))]
        #name {
            source: sqlx::Error,
            #[snafu(implicit)]
            location: snafu::Location
        },
    }
}

fn option_error_variant(data: &StructData, action: &str, doc: &str) -> TokenStream {
    let name = Ident::new(
        &format!("{}{action}Error", data.struct_name),
        Span::call_site(),
    );

    quote! {
        #[snafu(display(#doc))]
        #name {
            #[snafu(implicit)]
            location: snafu::Location
        },
    }
}

pub fn get_error_type(data: &StructData) -> TokenStream {
    if data.gen_snafu {
        let name = Ident::new(&format!("{}SqlError", data.struct_name), Span::call_site());

        quote! {#name}
    } else {
        quote! {sqlx::Error}
    }
}

pub fn get_snafu_type(data: &StructData, action: &str) -> Option<TokenStream> {
    if data.gen_snafu {
        let name = Ident::new(
            &format!("{}{action}Snafu", data.struct_name),
            Span::call_site(),
        );

        Some(quote! {#name})
    } else {
        None
    }
}
