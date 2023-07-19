use crate::logging;
use crate::mainloop::main_loop;
use clap::Parser;

/// A full-terminal interactive shell
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Options {}

pub async fn run() -> anyhow::Result<()> {
    let _ = Options::parse();
    let paths = application_paths::application_paths!();
    let logpath = paths.log_file()?;
    logging::init(logpath)?;
    main_loop().await
}
