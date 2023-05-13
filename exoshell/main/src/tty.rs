use anyhow::anyhow;
use crossterm::tty::IsTty;
use std::io::Stdout;

pub(crate) fn get() -> anyhow::Result<Stdout> {
    let stdout = std::io::stdout();

    if stdout.is_tty() {
        Ok(stdout)
    } else {
        Err(anyhow!("stdout not a tty"))
    }
}
