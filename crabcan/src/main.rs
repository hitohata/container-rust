mod cli;
mod config;
mod container;
mod errors;

use errors::exit_with_retcode;
use std::process::exit;

fn main() {
    match cli::parse_args() {
        Ok(arg) => {
            log::info!("{:?}", arg);
            exit_with_retcode(Ok(()))
        }
        Err(e) => {
            log::error!("Error while parsing arguments:\n\t{}", e);
            exit(e.get_retcode())
        }
    }
}
