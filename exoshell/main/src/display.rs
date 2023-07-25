use crate::glyphs::Glyph;
use crossterm::terminal;
use std::io::{Stdout, Write};

const WELCOME: &str = "🐢 Entering the exoshell…\n";
const GOODBYE: &str = "🐢 Until next time! 👋\n";

#[derive(Debug)]
pub(crate) struct Display {
    stdout: Stdout,
}

impl Display {
    pub(crate) fn new() -> anyhow::Result<Self> {
        use crate::{screen, tty};

        let mut stdout = tty::get()?;
        stdout.write_all(WELCOME.as_bytes())?;
        screen::setup(&mut stdout)?;
        Ok(Display { stdout })
    }

    pub(crate) fn cleanup(&mut self) -> anyhow::Result<()> {
        crate::screen::exit(&mut self.stdout)
    }

    pub(crate) fn goodbye(&mut self) -> anyhow::Result<()> {
        self.stdout.write_all(GOODBYE.as_bytes())?;
        Ok(())
    }

    pub(crate) fn update(&mut self) -> anyhow::Result<()> {
        self.stdout.flush()?;
        Ok(())
    }

    pub(crate) fn write_row<G>(
        &mut self,
        row: u16,
        glyph: G,
        line: &str,
    ) -> anyhow::Result<&mut Self>
    where
        G: Glyph,
    {
        use crossterm::{cursor, style, QueueableCommand};

        assert!(!line.contains('\n'), "{line:?}");

        self.stdout
            .queue(cursor::MoveTo(0, row))?
            .queue(terminal::Clear(terminal::ClearType::CurrentLine))?
            .queue(style::SetBackgroundColor(style::Color::DarkCyan))?
            .write_all(CharBytes::new(glyph.glyph()).as_bytes())?;

        self.stdout
            .queue(style::SetBackgroundColor(style::Color::Reset))?
            .write_all(line.as_bytes())?;

        Ok(self)
    }
}

struct CharBytes {
    len: usize,
    buf: [u8; 4],
}

impl CharBytes {
    fn new(c: char) -> Self {
        let len = c.len_utf8();
        let mut buf = [0; 4];
        c.encode_utf8(&mut buf);
        CharBytes { len, buf }
    }

    fn as_bytes(&self) -> &[u8] {
        &self.buf[..self.len]
    }
}
