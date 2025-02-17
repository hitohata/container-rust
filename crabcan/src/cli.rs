use std::path::PathBuf;
use structopt::StructOpt;

use crate::errors::ErrCode;

#[derive(Debug, StructOpt)]
#[structopt(name = "crabcan", about = "A simple container in Rust.")]
pub struct Args {
    /// Activate debug mode
    #[structopt(short, long)]
    debug: bool,

    /// Command to execute inside the container
    #[structopt(short, long)]
    pub command: String,

    /// User ID to create inside the container
    #[structopt(short, long)]
    pub uid: u32,

    /// Directory to mount as root of the container
    #[structopt(parse(from_os_str), short = "m", long = "mount")]
    pub mount_dir: PathBuf,
}

pub fn parse_args() -> Result<Args, ErrCode> {
    let args = Args::from_args();

    if !args.mount_dir.exists() || !args.mount_dir.is_dir() {
        return Err(ErrCode::ArgumentInvalid("mount"));
    }

    if args.debug {
        setup_logging(log::LevelFilter::Debug);
    } else {
        setup_logging(log::LevelFilter::Info);
    }

    Ok(args)
}

pub fn setup_logging(level: log::LevelFilter) {
    env_logger::Builder::from_default_env()
        .format_timestamp(Some(env_logger::TimestampPrecision::Seconds))
        .filter_level(level)
        .init();
}
