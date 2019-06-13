use crate::cli::FactsCommand;

use crate::facts::Facts;

use serde_json::to_string_pretty;

use std::default::Default;

/// Dump out facts.
pub fn call(_config: FactsCommand) {
    println!(
        "{}",
        to_string_pretty(&Facts::default().discover()).unwrap()
    );
}
