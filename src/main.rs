mod cli;
mod cmd;
mod facts;
mod logging;

use structopt::StructOpt;

fn main() {
    // parse CLI args
    let args = cli::Args::from_args();

    // initialize logging
    logging::init(&args);

    // execute the given command
    match args.command {
        cli::Command::Facts(f) => cmd::facts::call(&f),
        cli::Command::Render(r) => cmd::render::call(&r),
    }
}
