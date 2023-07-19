use crate::display::Display;
use crate::event::Event;
use crate::termsize::term_size;
use crate::Runner;
use exoshell_aui::{Pane, Rect};

const HEADER_INDICATOR: char = '>';
const VERTICAL_TRUNCATION: char = '⋮';

pub(crate) struct UI {
    runner: Runner,
    display: Display,
    inbuf: String,
    size: Rect<u16>,
}

impl UI {
    pub(crate) fn new(runner: Runner) -> anyhow::Result<Self> {
        let display = Display::new()?;
        let inbuf = String::new();
        let size = term_size()?;

        let mut me = UI {
            runner,
            display,
            inbuf,
            size,
        };
        me.display_prompt()?;
        Ok(me)
    }

    pub(crate) fn cleanup(&mut self) -> anyhow::Result<()> {
        self.display.cleanup()
    }

    pub(crate) fn goodbye(&mut self) -> anyhow::Result<()> {
        self.display.goodbye()
    }

    pub(crate) fn handle_event(&mut self, event: Event) -> anyhow::Result<()> {
        use Event::*;

        match event {
            Tick(_) => Ok(()),

            Terminal(evres) => {
                let event = evres?;
                self.handle_ct_event(event)
            }
            Child(event) => {
                self.runner.handle_event(event)?;
                self.display_runs()
            }
        }
    }

    fn handle_ct_event(&mut self, ev: crossterm::event::Event) -> anyhow::Result<()> {
        use crossterm::event::{Event::Key, KeyEvent, KeyEventKind};

        match ev {
            Key(KeyEvent {
                code,
                kind: KeyEventKind::Press,
                ..
            }) => {
                use crossterm::event::KeyCode::{Char, Enter};

                match code {
                    Enter => {
                        self.runner.handle_command(&self.inbuf)?;
                        self.inbuf.clear();
                        self.display_runs()?;
                        self.display_prompt()
                    }
                    Char(c) => {
                        self.inbuf.push(c);
                        self.display_prompt()
                    }
                    _ => Ok(()),
                }
            }
            _ => Ok(()),
        }
    }

    fn display_runs(&mut self) -> anyhow::Result<()> {
        use crate::glyphs::Glyph;

        #[derive(Debug)]
        enum FullRowMeta {
            PaneMeta(exoshell_runner::PaneMeta),
            RowTruncation,
        }
        use FullRowMeta::*;
        impl Glyph for FullRowMeta {
            fn glyph(&self) -> char {
                match self {
                    PaneMeta(m) => m.glyph(),
                    RowTruncation => VERTICAL_TRUNCATION,
                }
            }
        }

        self.update_size()?;
        tracing::debug!(?self.size);

        // Subtract 1 for the prompt row:
        let mut row_bottom = last_row(self.size) - 1;

        for run in self.runner.runs().rev() {
            let mut pane = Pane::from((self.size.width() - 1, row_bottom));
            run.render_into(&mut pane)?;
            row_bottom -= u16::try_from(pane.content_len()).unwrap();
            for (rowdelta, rowinfo) in pane.iter().enumerate() {
                let row = row_bottom + u16::try_from(rowdelta).unwrap();
                let (meta, line) = if let Some((pm, line)) = rowinfo {
                    (PaneMeta(pm), line)
                } else {
                    (RowTruncation, "")
                };
                self.display.write_row(row, meta, line)?;
            }
            if row_bottom == 0 {
                break;
            }
        }

        self.display_prompt()
    }

    fn display_prompt(&mut self) -> anyhow::Result<()> {
        let inbuf = &self.inbuf;
        self.display
            .write_row(last_row(self.size), HEADER_INDICATOR, inbuf)?
            .update()?;
        Ok(())
    }

    fn update_size(&mut self) -> anyhow::Result<()> {
        self.size = term_size()?;
        Ok(())
    }
}

fn last_row(termsize: Rect<u16>) -> u16 {
    termsize.height() - 1
}
