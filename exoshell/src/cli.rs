use crate::mainloop::main_loop;
use clap::Parser;

/// A full-terminal interactive shell
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Options {}

pub async fn run() -> anyhow::Result<()> {
    let _ = Options::parse();
    main_loop().await
}
