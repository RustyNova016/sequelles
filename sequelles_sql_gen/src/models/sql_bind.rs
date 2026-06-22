

use crate::models::dialects::PostgreSQLDialect;
use crate::models::dialects::SqliteDialect;

#[derive(Debug, Default)]
pub struct SqlBinds {
    binds: Vec<(u64, String)>,
}

impl SqlBinds {
    pub fn iter(&self) -> std::slice::Iter<'_, (u64, String)> {
        self.binds.iter()
    }

    pub fn create_numbered_bind(&mut self, name: String) -> String {
        let num = self.binds.len() + 1;
        self.binds.push((num as u64, name));
        format!("${num}")
    }
}

pub trait HasBinds {
    fn create_bind(binds: &mut SqlBinds, name: String) -> String;
}

impl HasBinds for PostgreSQLDialect {
    fn create_bind(binds: &mut SqlBinds, name: String) -> String {
        binds.create_numbered_bind(name)
    }
}

impl HasBinds for SqliteDialect {
    fn create_bind(binds: &mut SqlBinds, name: String) -> String {
        binds.create_numbered_bind(name)
    }
}