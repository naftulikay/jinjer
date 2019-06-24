pub mod plugins;

use futures::Future;

use serde_json::Map;
use serde_json::Value;

use std::collections::HashMap;
use std::default::Default;
use std::io;
use std::iter::once;

pub type FactSet = Map<String, Value>;

/// A future that, when executed, returns a set of facts.
pub type FactSetFuture = Future<Item=FactSet, Error=io::Error>;

/// A registry for asynchronous plugins.
pub type AsyncPluginRegistry = HashMap<String, Box<dyn AsyncFactPlugin>>;

/// A registry for synchronous plugins.
pub type PluginRegistry = HashMap<String, Box<dyn FactPlugin>>;

pub struct Facts {
    plugins: PluginRegistry,
    async_plugins: AsyncPluginRegistry,
}

impl Facts {
    pub fn new() -> Self {
        Self { ..Default::default() }
    }

    /// Discover all available facts via the registered fact plugins.
    ///
    /// This operation will take some time as it fetches facts from all available providers.
    pub fn discover(&self) -> FactSet {
        let mut result = FactSet::new();

        for factset in once(self.discover_sync()).chain(once(self.discover_async())) {
            result.extend(factset);
        }

        result
    }

    /// Discover synchronous fact plugins.
    /// 
    /// FIXME either use rayon for parallelism here or just use futures everywhere
    fn discover_sync(&self) -> FactSet {
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

    /// Discover asynchronous fact plugins.
    fn discover_async(&self) -> FactSet {
        FactSet::new()
    }

    pub fn register(&mut self, id: &str, plugin: Box<dyn FactPlugin>) {
        self.plugins.insert(id.to_string(), plugin);
    }

    pub fn register_async(&mut self, id: &str, plugin: Box<dyn AsyncFactPlugin>) {
        self.async_plugins.insert(id.to_string(), plugin);        
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

/// A plugin which discovers facts synchronously.
pub trait FactPlugin {
    /// Execute the plugin, discovering and returning all available facts.
    fn discover(&self) -> Result<FactSet, io::Error>;
}

/// A plugin which discovers facts asynchronously.
pub trait AsyncFactPlugin {
    /// Execute the plugin, yielding a future that can be polled for readiness.
    fn discover(&self) -> Future<Item=FactSet, Error=io::Error>;

}
