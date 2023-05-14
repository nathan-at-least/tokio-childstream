use std::io::Stdout;

#[allow(dead_code)]
pub(crate) fn display(stdout: &mut Stdout) -> anyhow::Result<()> {
    use crossterm::{cursor, style, terminal, QueueableCommand};
    use std::io::Write;

    let (columns, rows) = terminal::size()?;
    stdout
        .queue(cursor::MoveTo(0, rows - 2))?
        .queue(style::SetBackgroundColor(style::Color::DarkGreen))?;

    for _ in 0..columns {
        stdout.write_all(b" ")?;
    }
    Ok(())
}
