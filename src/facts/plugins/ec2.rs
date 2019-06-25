mod metadata;

use actix_web::client::Client;
use actix_web::http::header::HeaderValue;

use futures::Future;

use crate::facts::FactPlugin;
use crate::facts::FactSet;

use http::StatusCode;

use log;

use std::default::Default;

use std::env;

use std::io;

lazy_static! {
    pub static ref EC2_METADATA_URL: String = format!(
        "http://{}/",
        env::var("EC2_METADATA_HOST").unwrap_or("169.254.169.254".to_string())
    );
}

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
    fn discover(&self) -> Box<Future<Item = FactSet, Error = io::Error>> {
        let f = Client::default()
            .get(EC2_METADATA_URL.as_str())
            .header("User-Agent", "jinjer")
            .send()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("{:?}", e)))
            .map(|resp| {
                log::debug!("Reached the EC2 metadata server: {:?}", resp);

                if resp.status() != StatusCode::OK {
                    log::debug!(
                        "Reached the EC2 metadata service, but received an error response."
                    );

                    // return Err(io::Error::new(io::ErrorKind::NotFound, "HTTP error reaching EC2 metadata service."));
                }

                FactSet::new()
            });

        Box::new(f)
    }
}
