use crate::display::Display;
use crate::event::Event;
use crate::termsize::TermSize;
use crate::Runner;
use exoshell_runner::{Run, Status};

const HEADER_INDICATOR: char = '>';
const VERTICAL_TRUNCATION: char = '⋮';

pub(crate) struct UI {
    runner: Runner,
    display: Display,
    inbuf: String,
    size: TermSize,
}

impl UI {
    pub(crate) fn new(runner: Runner) -> anyhow::Result<Self> {
        let display = Display::new()?;
        let inbuf = String::new();
        let size = TermSize::new()?;

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
        self.update_size()?;

        let mut row_bottom = self.size.last_row();

        for run in self.runner.runs().rev() {
            // BUG: log_length is lines != len of formatted log lines for long lines
            let len_full = run.log_length() + 1; // 1 for header line

            let (row_top, truncate) = if len_full > usize::from(row_bottom) {
                (0, true)
            } else {
                (row_bottom - u16::try_from(len_full)?, false)
            };

            self.display
                .move_to_row(row_top)?
                .write_glyph_line(HEADER_INDICATOR, &format_header(run, self.size))?;

            if truncate {
                // -1 from half includes 1 row for header and 1 for mid-truncation
                let len_half = (row_bottom - row_top) / 2 - 1;

                self.display
                    .write_glyph_lines(
                        run.format_log(self.size.cols_log())
                            .take(usize::from(len_half)),
                    )?
                    .write_glyph_line(VERTICAL_TRUNCATION, "")?
                    .write_glyph_lines(
                        run.format_log(self.size.cols_log())
                            .skip(run.log_length() - usize::from(len_half)),
                    )?;
            } else {
                self.display
                    .write_glyph_lines(run.format_log(self.size.cols_log()))?;
            }

            row_bottom = row_top;
            if row_bottom == 0 {
                break;
            }
        }

        self.display_prompt()
    }

    fn display_prompt(&mut self) -> anyhow::Result<()> {
        let inbuf = &self.inbuf;
        self.display
            .move_to_row(self.size.last_row())?
            .write_glyph_line(HEADER_INDICATOR, inbuf)?
            .update()?;
        Ok(())
    }

    fn update_size(&mut self) -> anyhow::Result<()> {
        self.size = TermSize::new()?;
        Ok(())
    }
}

fn format_header(run: &Run, size: TermSize) -> String {
    let status = status_info(run);
    let cutoff = usize::from(size.cols_log()) - status.chars().count();
    let cmdtext = run.command();
    let mut s = String::new();
    if cmdtext.chars().count() > cutoff {
        s.extend(cmdtext.chars().take(cutoff - 1));
        s.push('…');
    } else {
        s.push_str(cmdtext);
        for _ in cmdtext.len()..cutoff {
            s.push(' ');
        }
    }
    s.push_str(&status);
    assert_eq!(s.len(), usize::from(size.cols_log()));
    s
}

fn status_info(run: &Run) -> String {
    if let Status::Exited(es) = run.status() {
        format!(
            "[exit {}]",
            es.code()
                .map(|n| n.to_string())
                .unwrap_or_else(|| "?".to_string())
        )
    } else {
        "".to_string()
    }
}
