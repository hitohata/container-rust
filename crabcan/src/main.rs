mod cli;
mod config;
mod container;
mod errors;

use errors::exit_with_retcode;
use std::process::exit;

fn main() {
    match cli::parse_args() {
        Ok(args) => {
            log::info!("{:?}", args);
            exit_with_retcode(container::Container::start(args));
        }
        Err(e) => {
            log::error!("Error while parsing arguments:\n\t{}", e);
            exit(e.get_retcode())
        }
    }
}
