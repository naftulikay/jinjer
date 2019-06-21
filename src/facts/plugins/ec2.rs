mod metadata;

use actix_web::client::Client;
use actix_web::http::header::HeaderValue;

use crate::facts::FactPlugin;
use crate::facts::FactSet;

use futures::Future;

use http::StatusCode;

use log;

use serde_json::Value;

use std::default::Default;

use std::env;

use std::io;

lazy_static! {
    pub static ref EC2_METADATA_URL: String = format!("http://{}/", env::var("EC2_METADATA_HOST").unwrap_or("169.254.169.254".to_string()));
}

/// A fact plugin for EC2-related data.
///
/// This plugin will produce EC2 metadata and EC2 tag facts.
pub struct Ec2Plugin;

impl Ec2Plugin {
    pub fn new() -> Self {
        Self::default()
    }

    /// Determine whether we are currently running in EC2.
    /// 
    /// At present, we send a HEAD request to the EC2 metadata service and examine the `Server` header to determine
    /// whether we're in EC2. If the `Server` header is `EC2ws`, we're in EC2.
    fn is_ec2(&self) -> bool {
        log::debug!("Checking the EC2 metadata server to detect whether we're in EC2...");

        let fut = Client::default().head(EC2_METADATA_URL.as_str())
            .header("User-Agent", "jinjer")
            .send();

        match fut.wait() {
            Ok(response) => {
                match response.status() {
                    StatusCode::OK => {
                        response.headers().get("Server").unwrap_or(&HeaderValue::from_static("Unknown")) == HeaderValue::from_static("EC2ws")
                    },
                    _ => {
                        log::debug!("Received non-200 response: {:?}", response);
                        false
                    }
                }
            },
            Err(e) => {
                log::debug!("Unable to query EC2 metadata service: {:?}", e);   
                false
            }
        }
    }
}

impl Default for Ec2Plugin {
    fn default() -> Self {
        Self
    }
}

impl FactPlugin for Ec2Plugin {
    fn discover(&self) -> Result<FactSet, io::Error> {
        if !self.is_ec2() {
            return Err(io::Error::new(io::ErrorKind::Other, "Not running in EC2."));
        }

        log::info!("Discovering EC2 facts...");

        let mut r = FactSet::new();
        let meta = metadata::Ec2MetadataPlugin::new();

        // if ec2 metadata discovery fails, then we return that failure and ec2 facts are not
        // returned
        r.insert("metadata".to_string(), Value::Object(meta.discover()?));

        Ok(r)
    }
}
