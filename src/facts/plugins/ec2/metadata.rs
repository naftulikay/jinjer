use crate::facts::FactPlugin;
use crate::facts::FactSet;

use std::default::Default;

use std::io;

/// An EC2 instance metadata fact plugin.
///
/// This plugin harvests all available facts present in the latest API revision of the EC2 metadata
/// service.
pub struct Ec2MetadataPlugin;

impl Ec2MetadataPlugin {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Ec2MetadataPlugin {
    fn default() -> Self {
        Self
    }
}

impl FactPlugin for Ec2MetadataPlugin {
    fn discover(&self) -> Result<FactSet, io::Error> {
        Ok(FactSet::new())
    }
}
