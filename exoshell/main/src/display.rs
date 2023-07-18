use crate::glyphs::Glyph;
use crossterm::terminal;
use std::io::{Stdout, Write};

const WELCOME: &str = "🐢 Entering the exoshell…\n";
const GOODBYE: &str = "🐢 Until next time! 👋\n";

#[derive(Debug)]
pub(crate) struct Display {
    stdout: Stdout,
    // We must track row separately from `crossterm` because `crossterm::cursor::position` may fail.
    row: u16,
}

impl Display {
    pub(crate) fn new() -> anyhow::Result<Self> {
        use crate::{screen, tty};

        let mut stdout = tty::get()?;
        stdout.write_all(WELCOME.as_bytes())?;
        screen::setup(&mut stdout)?;
        Ok(Display { stdout, row: 0 })
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

    pub(crate) fn move_to_row(&mut self, row: u16) -> anyhow::Result<&mut Self> {
        use crossterm::{cursor, QueueableCommand};

        self.row = row;
        self.stdout.queue(cursor::MoveTo(0, self.row))?;
        Ok(self)
    }

    pub(crate) fn write_glyph_line<G>(&mut self, glyph: G, line: &str) -> anyhow::Result<&mut Self>
    where
        G: Glyph,
    {
        use crate::termsize::TermSize;
        use crossterm::{style, QueueableCommand};

        let tsize = TermSize::new()?;

        assert!(!line.contains('\n'), "{line:?}");
        assert!(
            line.chars().count() <= usize::from(tsize.cols_log()),
            "{line:?}"
        );

        self.stdout
            .queue(terminal::Clear(terminal::ClearType::CurrentLine))?
            .queue(style::SetBackgroundColor(style::Color::DarkCyan))?
            .write_all(CharBytes::new(glyph.glyph()).as_bytes())?;

        self.stdout
            .queue(style::SetBackgroundColor(style::Color::Reset))?
            .write_all(line.as_bytes())?;

        let size = TermSize::new()?;
        if self.row < size.last_row() {
            self.move_to_row(self.row + 1)?;
        }

        Ok(self)
    }

    pub(crate) fn write_glyph_lines<'a, I, G>(&mut self, lines: I) -> anyhow::Result<&mut Self>
    where
        I: IntoIterator<Item = (G, &'a str)>,
        G: Glyph,
    {
        for (g, s) in lines {
            self.write_glyph_line(g, s)?;
        }
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
