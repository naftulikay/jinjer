#[macro_use]
extern crate lazy_static;

mod cli;
mod cmd;
mod facts;
mod logging;

use actix_rt::System;

use parking_lot::Once;

use structopt::StructOpt;

static CREATE_ACTIX_RUNTIME: Once = Once::new();

fn main() {
    // parse CLI args
    let args = cli::Args::from_args();

    // initialize logging
    logging::init(&args);

    // setup actix system runtime
    CREATE_ACTIX_RUNTIME.call_once(|| {
        log::info!("Setting up Actix system...");
        let _ = System::new("jinjer");
    });

    // execute the given command
    match args.command {
        cli::Command::Facts(f) => cmd::facts::call(f),
        cli::Command::Render(r) => cmd::render::call(r),
    }
}
