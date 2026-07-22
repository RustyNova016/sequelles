use std::rc::Rc;

use sequelles_sql_gen::models::schema::field::Column;
use sequelles_sql_gen::models::schema::table::Table;
use syn::DeriveInput;
use syn::Field;
use syn::Fields;
use syn::Ident;

use crate::models::attributes::field_attributes::ColumnAtribute;
use crate::models::attributes::table_attributes::TableAtribute;
use crate::models::attributes::table_attributes::TableStructAtribute;
use crate::models::attributes::table_attributes::TableUniqueKey;

pub struct StructData {
    pub struct_name: Ident,
    pub struct_fields: Vec<Field>,

    pub postgres: bool,
    pub sqlite: bool,

    pub gen_unique_keys: bool,
    pub gen_delete: bool,
    pub gen_update: bool,
    pub gen_select: bool,
    pub gen_select_unique: bool,
    pub gen_insert: bool,
    pub gen_insert_struct: bool,
    pub gen_dyn_insert_struct: bool,
    pub gen_upsert: bool,
    pub gen_snafu: bool,
    pub gen_selsert: bool,

    pub table: Table,
}

impl StructData {
    pub fn parse_struct(ast: &mut DeriveInput) -> Self {
        let struct_ident = ast.ident.clone();
        let tattr: TableStructAtribute =
            deluxe::extract_attributes(ast).expect("Couldn't parse struct attributes");

        let syn::Data::Struct(ast) = &mut ast.data else {
            panic!("The derive macro should be on a struct");
        };

        let mut fields = Vec::new();
        for field in ast.fields.iter_mut() {
            fields.push(field.to_owned());
        }

        let table = Self::parse_table(tattr.table_data, struct_ident.to_string(), &mut ast.fields);

        Self {
            struct_name: struct_ident,
            gen_delete: tattr.delete,
            gen_update: tattr.update,
            gen_insert: tattr.insert || tattr.selsert,
            gen_select: tattr.select,
            gen_select_unique: tattr.select_unique,
            gen_insert_struct: tattr.insert_struct,
            gen_dyn_insert_struct: tattr.dyn_insert_struct,
            gen_unique_keys: tattr.select_unique || tattr.selsert,
            gen_upsert: tattr.upsert,
            gen_selsert: tattr.selsert,
            gen_snafu: tattr.snafu,
            postgres: tattr.postgres,
            sqlite: tattr.sqlite,
            struct_fields: fields,
            table,
        }
    }

    pub fn parse_table(tattr: TableAtribute, struct_name: String, fields: &mut Fields) -> Table {
        let db_name = tattr.db_name.as_ref().cloned().unwrap_or(struct_name);

        // Create the table
        let mut table = Table::new(db_name, None);

        for field in fields.iter_mut() {
            Self::parse_fields(&mut table, field)
        }

        if let Some(pk) = tattr.primary_key.first() {
            Self::add_table_unique_key(&mut table, pk.clone(), true);
        }

        for ukey in tattr.unique_keys {
            Self::add_table_unique_key(&mut table, ukey, false);
        }

        table
    }

    pub fn parse_fields(table: &mut Table, field: &mut Field) {
        let rust_name = field.ident.as_ref().unwrap().to_string();
        let cattr: ColumnAtribute = deluxe::extract_attributes(field).unwrap();

        let sql_name = cattr.db_name.clone().unwrap_or_else(|| rust_name.clone());

        let col = Rc::new(Column {
            sql_name,
            rust_name: rust_name,
            auto_increment: cattr.auto_increment,
            default: cattr.default,
        });

        table.fields.push(&col);
    }

    fn add_table_unique_key(table: &mut Table, ukey: TableUniqueKey, pk: bool) {
        for field in ukey.columns {
            let field = table
                .fields
                .get_field_with_rust_name(&field.to_string())
                .cloned()
                .expect(&format!(
                    "Cannot find field {field} for unique key {}",
                    ukey.key_name
                ));
            table.insert_field_for_key(&ukey.key_name, &field, pk);
        }
    }

    pub fn get_field_by_name(&self, name: &str) -> Option<&Field> {
        self.struct_fields
            .iter()
            .find(|f| f.ident.as_ref().is_some_and(|i| &i.to_string() == name))
    }
}
