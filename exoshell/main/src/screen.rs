use crossterm::{cursor, terminal, QueueableCommand};
use std::io::Stdout;

pub(crate) fn setup(stdout: &mut Stdout) -> anyhow::Result<()> {
    use terminal::{Clear, ClearType::All, EnterAlternateScreen};

    terminal::enable_raw_mode()?;

    stdout
        .queue(EnterAlternateScreen)?
        .queue(Clear(All))?
        .queue(cursor::SetCursorStyle::BlinkingBlock)?;
    Ok(())
}

pub(crate) fn exit(stdout: &mut Stdout) -> anyhow::Result<()> {
    use std::io::Write;

    stdout
        .queue(cursor::SetCursorStyle::DefaultUserShape)?
        .queue(terminal::LeaveAlternateScreen)?
        .flush()?;

    terminal::disable_raw_mode()?;
    Ok(())
}
