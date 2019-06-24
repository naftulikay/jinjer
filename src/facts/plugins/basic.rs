use crate::facts::FactPlugin;
use crate::facts::FactSet;

use futures::Future;

use log;

use num_cpus;

use serde_json::Number;
use serde_json::Value;

use std::default::Default;
use std::io;

/// A fact plugin providing a basic set of system facts.
pub struct BasicPlugin {}

impl BasicPlugin {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for BasicPlugin {
    fn default() -> Self {
        Self {}
    }
}

impl FactPlugin for BasicPlugin {
    fn discover(&self) -> Box<Future<Item = FactSet, Error = io::Error>> {
        Box::new(futures::lazy(|| {
            log::info!("Discovering basic facts...");

            let mut f = FactSet::new();

            f.insert(
                "cpu_cores".to_string(),
                Value::Object({
                    let mut p = FactSet::new();

                    p.insert(
                        "logical".to_string(),
                        Value::Number(Number::from(num_cpus::get())),
                    );
                    p.insert(
                        "physical".to_string(),
                        Value::Number(Number::from(num_cpus::get_physical())),
                    );

                    p
                }),
            );

            Ok(f)
        }))
    }
}
