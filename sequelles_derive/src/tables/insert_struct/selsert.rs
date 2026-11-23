use convert_case::Case;
use convert_case::Casing;
use proc_macro2::Ident;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use sequelles_sql_gen::models::dialects::PostgreSQLDialect;
use sequelles_sql_gen::models::dialects::SqliteDialect;

use crate::models::database_data::StructData;
use crate::models::sql_dialects::SqlxConnectionType;
use crate::tables::error::get_error_type;
use crate::tables::error::get_snafu_type;
use crate::tables::insert_struct::create_struct::CreateInsertStruct;

impl CreateInsertStruct {
    pub fn impl_selsert_trait(data: &StructData) -> TokenStream {
        if !data.gen_selsert {
            return quote! {};
        }

        let mut tokens = quote! {};

        if data.postgres {
            let imple = impl_selsert_trait_for_dialect::<PostgreSQLDialect>(&data);
            tokens = quote! {#tokens #imple};
        }

        if data.sqlite {
            let imple = impl_selsert_trait_for_dialect::<SqliteDialect>(&data);
            tokens = quote! {#tokens #imple};
        }

        tokens
    }
}

fn impl_selsert_trait_for_dialect<L>(data: &StructData) -> TokenStream
where
    L: SqlxConnectionType,
{
    let row_struct = &data.struct_name;
    let insert_struct = CreateInsertStruct::get_struct_ident(data);
    let for_conn = L::get_mut_connection();
    let error = get_error_type(data);

    let ukey = data
        .table
        .unique_keys()
        .iter()
        .find(|key| !key.has_default_column())
        .expect("Cannot generate selsert: Cannot find an unique key without default");

    let function_name = Ident::new(
        &format!(
            "get_{}_unique_key",
            ukey.name.to_string().to_case(Case::Snake)
        ),
        Span::call_site(),
    );

    let context_trans =
        get_snafu_type(data, "SelsertTransaction").map(|name| quote! {.context(#name)});
    let context_not_found = get_snafu_type(data, "SelsertNotFound")
        .map(|name| quote! {.context(#name)})
        .unwrap_or_else(|| quote! {.expect("Couldn't find the row after inserting it.")});

    quote! {
        impl sequelles::Selsert<#for_conn> for #insert_struct {
            type Output = #row_struct;
            type Error = #error;

            async fn selsert(
                &self,
                conn: #for_conn
            ) -> Result<#row_struct, Self::Error>
            {
                use sequelles::sqlx::Acquire as _;
                use sequelles::InsertOrIgnore as _;
                use sequelles::SelectUnique as _;
                use sequelles::snafu::ResultExt as _;
                use sequelles::snafu::OptionExt as _;

                let mut trans = conn.begin()
                    .await
                    #context_trans?;
                match self.insert_or_ignore(&mut *trans).await? {
                    Some(res) => {
                        trans.commit()
                            .await
                            #context_trans?;
                        Ok::<#row_struct, Self::Error>(res)
                    },
                    None => {
                        let res = #row_struct::select_unique(&mut *trans, self.#function_name())
                            .await?
                            #context_not_found?;
                        trans.commit()
                            .await
                            #context_trans?;
                        Ok(res)
                    }
                }
            }
        }
    }
}
