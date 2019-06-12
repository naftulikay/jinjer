pub mod plugins;

use serde_json::Map;
use serde_json::Value;

use std::collections::HashMap;

use std::default::Default;

use std::io;

pub type FactSet = Map<String, Value>;

pub type PluginRegistry = HashMap<String, Box<dyn FactPlugin>>;

pub struct Facts {
    plugins: PluginRegistry,
}

impl Facts {
    pub fn new() -> Self {
        Self {
            plugins: PluginRegistry::new(),
        }
    }

    /// Discover all available facts via the registered fact plugins.
    ///
    /// This operation will take some time as it fetches facts from all available providers.
    pub fn discover(&self) -> FactSet {
        let mut r = FactSet::new();

        for (name, plugin) in self.plugins.iter() {
            match plugin.discover() {
                Ok(f) => {
                    r.insert(name.to_string(), Value::Object(f));
                }
                Err(e) => log::warn!("Discovery failed for {}: {}", name, e),
            }
        }

        r
    }

    pub fn register(&mut self, id: &str, plugin: Box<dyn FactPlugin>) {
        self.plugins.insert(id.to_string(), plugin);
    }
}

impl Default for Facts {
    fn default() -> Self {
        let mut f = Facts::new();

        // install plugins
        f.register("basic", Box::new(plugins::basic::BasicPlugin::new()));
        f.register("env", Box::new(plugins::env::EnvPlugin::new()));

        f
    }
}

pub trait FactPlugin {
    /// Execute the plugin, discovering and returning all available facts.
    fn discover(&self) -> Result<FactSet, io::Error>;
}
