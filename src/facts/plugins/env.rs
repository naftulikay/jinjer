use crate::facts::FactPlugin;
use crate::facts::FactSet;

use serde_json::Value;

use std::default::Default;

use std::io;

/// A fact plugin for environment variables.
pub struct EnvPlugin {}

impl EnvPlugin {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for EnvPlugin {
    fn default() -> Self {
        Self {}
    }
}

impl FactPlugin for EnvPlugin {
    fn discover(&self) -> Result<FactSet, io::Error> {
        log::info!("Discovering facts from environment variables...");

        let mut f = FactSet::new();

        for (key, value) in std::env::vars() {
            f.insert(key, Value::String(value));
        }

        Ok(f)
    }
}
