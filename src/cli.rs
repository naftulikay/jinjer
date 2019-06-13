use std::path::PathBuf;

use structopt::StructOpt;

/// A CLI utility for rendering Jinja-like templates using the Tera template engine.
#[derive(Debug, StructOpt)]
#[structopt(author = "")]
pub struct Args {
    /// The subcommand to execute.
    #[structopt(subcommand)]
    pub command: Command,
    /// Logging verbosity. By default, it is set to ERROR, pass -v multiple times to increase
    /// verbosity.
    #[structopt(short = "v", parse(from_occurrences))]
    pub verbosity: u8,
}

#[derive(Debug, StructOpt)]
pub enum Command {
    /// Dump all detected facts to standard output. Useful for understanding which facts are
    /// available at runtime.
    #[structopt(name = "facts")]
    Facts(FactsCommand),
    /// Render one or more templates to standard output or a file.
    #[structopt(name = "render")]
    Render(RenderCommand),
}

#[derive(Debug, StructOpt)]
pub struct FactsCommand {}

#[derive(Debug, StructOpt)]
pub struct RenderCommand {
    /// Enable HTML auto-escaping of templates in the template renderer. By default, output is not
    /// safe for HTML.
    #[structopt(short = "e", long = "auto-escape")]
    pub autoescape: bool,
    /// Render output to a file rather than to standard output.
    #[structopt(short = "o", long = "output")]
    pub output_file: Option<PathBuf>,
    /// A list of template files to render in order.
    pub template_files: Vec<PathBuf>,
}
