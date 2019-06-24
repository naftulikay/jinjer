use actix_rt::System;
use actix_web::client::Client;

use crate::facts::FactPlugin;
use crate::facts::FactSet;

use futures::future::lazy;
use futures::future::Future;

use log;

use std::default::Default;

use std::env;

use std::io;

lazy_static! {
    static ref METADATA_URL: String = format!(
        "http://{}/latest/meta-data/",
        env::var("EC2_METADATA_HOST").unwrap_or("169.254.169.254".to_string())
    );
}

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
        log::info!("Discovering EC2 metadata facts...");

        System::new("ec2-metadata")
            .block_on(lazy(|| {
                Client::default()
                    .get(METADATA_URL.as_str())
                    .header("User-Agent", "jinjer")
                    .send()
                    .map_err(|e| log::error!("Error: {:?}", e))
                    .and_then(|response| {
                        log::info!("Response: {:?}", response);
                        Ok(())
                    })
            }))
            .unwrap();

        Ok(FactSet::new())
    }
}
