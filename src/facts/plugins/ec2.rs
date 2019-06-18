mod metadata;

use crate::facts::FactPlugin;
use crate::facts::FactSet;

use serde_json::Value;

use std::default::Default;

use std::io;

/// A fact plugin for EC2-related data.
///
/// This plugin will produce EC2 metadata and EC2 tag facts.
pub struct Ec2Plugin;

impl Ec2Plugin {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Ec2Plugin {
    fn default() -> Self {
        Self
    }
}

impl FactPlugin for Ec2Plugin {
    fn discover(&self) -> Result<FactSet, io::Error> {
        let mut r = FactSet::new();
        let meta = metadata::Ec2MetadataPlugin::new();

        // if ec2 metadata discovery fails, then we return that failure and ec2 facts are not
        // returned
        r.insert("metadata".to_string(), Value::Object(meta.discover()?));

        Ok(r)
    }
}
