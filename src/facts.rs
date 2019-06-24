pub mod plugins;

use actix_rt::System;

use futures::Future;

use serde_json::Map;
use serde_json::Value;

use std::collections::HashMap;
use std::default::Default;
use std::io;

pub type FactSet = Map<String, Value>;

/// A registry for synchronous plugins.
pub type PluginRegistry = HashMap<String, Box<dyn FactPlugin>>;

pub struct Facts {
    plugins: PluginRegistry,
}

impl Facts {
    pub fn new() -> Self {
        Self { plugins: PluginRegistry::default() }
    }

    /// Discover all available facts via the registered fact plugins.
    ///
    /// This operation will take some time as it fetches facts from all available providers.
    pub fn discover(&self) -> FactSet {
        let mut result = FactSet::new();

        // FIXME this should be parallelized using a stream map over the futures
        for (name, plugin) in &self.plugins {
            log::info!("Executing asynchronous plugin {}", name);

            match System::new("jinjer").block_on(plugin.discover()) {
                Ok(v) => {
                    log::info!("Successfully determined {} facts.", name);
                    result.insert(name.to_string(), Value::Object(v));
                },
                Err(e) => {
                    match e.kind() {
                        io::ErrorKind::NotFound => log::debug!("Unable to resolve {} facts.", name),
                        _ => log::warn!("Unable to resolve facts for {}: {:?}", name, e),
                    };
                },
            };
        }

        result
    }

    /// Register a plugin.
    pub fn register(&mut self, id: &str, plugin: Box<dyn FactPlugin>) {
        self.plugins.insert(id.to_string(), plugin.into());
    }
}

impl Default for Facts {
    fn default() -> Self {
        let mut f = Facts::new();

        // install plugins
        f.register("basic", Box::new(plugins::basic::BasicPlugin::new()));
        f.register("ec2", Box::new(plugins::ec2::Ec2Plugin::new()));
        f.register("env", Box::new(plugins::env::EnvPlugin::new()));

        f
    }
}

/// A plugin which discovers facts asynchronously.
pub trait FactPlugin {
    /// Execute the plugin, yielding a future that can be polled for readiness.
    fn discover(&self) -> Box<dyn Future<Item=FactSet, Error=io::Error>>;
}
