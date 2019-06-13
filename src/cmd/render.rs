use crate::cli::RenderCommand;

use crate::facts::Facts;

use log;

use serde_json::Value;

use std::default::Default;

use std::fs;

use std::io;
use std::io::prelude::*;
use std::io::BufWriter;

use std::process;

use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

use tera::Context;
use tera::Tera;

/// Render the given templates.
pub fn call(config: RenderCommand) {
    // our facts to be provided to templates
    let context = Context::from_value(Value::Object(Facts::default().discover()))
        .expect("Unable to create a context from discovered facts.");

    // create a buffer around stdout/output file writes
    let mut writer = get_output_writer(&config);

    if config.template_files.len() == 0 {
        // if no template files are passed, render from stdin
        render_stdin(&config, context, &mut writer);
    } else {
        // user passed template files, render them in order
        render_template_files(&config, context, &mut writer);
    }
}

/// Open a writer either on standard output or to a given file passed in the config.
fn get_output_writer(config: &RenderCommand) -> BufWriter<Box<dyn Write>> {
    let output: Box<dyn Write> = match &config.output_file {
        Some(p) => Box::new(
            fs::OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&p)
                .unwrap_or_else(|e| {
                    log::error!(
                        "Unable to open output file {} for writing: {}",
                        p.display(),
                        e
                    );
                    process::exit(1);
                }),
        ),
        None => Box::new(io::stdout()),
    };

    // create a buffer around stdout/output file writes
    BufWriter::new(output)
}

/// Render the template present on standard input.
fn render_stdin(config: &RenderCommand, context: Context, writer: &mut Write) {
    log::debug!("Rendering template from standard input as no template files were passed...");
    let mut buffer = String::new();

    io::stdin().read_to_string(&mut buffer).unwrap_or_else(|e| {
        log::error!("Unable to read template from standard input: {}", e);
        process::exit(1);
    });

    match Tera::one_off(buffer.as_str(), context, config.autoescape) {
        Ok(s) => {
            write!(writer, "{}", s).unwrap_or_else(|e| {
                log::error!("Unable to write rendered template to output: {}", e);
                process::exit(1);
            });
        }
        Err(e) => {
            log::error!("Unable to render template from standard input: {:?}", e);
            process::exit(1);
        }
    }
}

/// Render a list of template files from the configuration.
fn render_template_files(config: &RenderCommand, context: Context, writer: &mut Write) {
    let success = AtomicBool::new(false);

    for template_file in &config.template_files {
        log::info!("Rendering {}...", template_file.display());

        match fs::read_to_string(template_file) {
            Ok(s) => match Tera::one_off(s.as_str(), context.clone(), config.autoescape) {
                // cool, we've been able to read the template file
                Ok(s) => {
                    // we have successfully rendered the template
                    write!(writer, "{}", s).unwrap_or_else(|e| {
                        log::error!("Unable to write rendered template to output: {}", e);
                        process::exit(1);
                    });

                    success.store(true, Ordering::SeqCst);
                }
                Err(e) => {
                    // rendering the template has failed
                    log::error!(
                        "Unable to render template file {}: {:?}",
                        template_file.display(),
                        e
                    );
                }
            },
            Err(e) => {
                // we weren't able to read the template file
                log::error!(
                    "Unable to read template file {}: {}",
                    template_file.display(),
                    e
                );
            }
        }
    }

    if !success.load(Ordering::SeqCst) {
        log::error!("Unable to successfully render any provided templates.");
        process::exit(1);
    }
}
