use crate::event::Event;
use crate::screen;
use crate::Runner;
use crossterm::terminal;
use std::io::{Stdout, Write};

const WELCOME: &str = "🐢 Entering the exoshell…\n";
const GOODBYE: &str = "🐢 Until next time! 👋\n";
const PROMPT: &str = "> ";

pub(crate) struct UI {
    runner: Runner,
    stdout: Stdout,
    inbuf: String,
}

impl UI {
    pub(crate) fn new(runner: Runner) -> anyhow::Result<Self> {
        let inbuf = String::new();
        let mut stdout = crate::tty::get()?;
        stdout.write_all(WELCOME.as_bytes())?;
        screen::setup(&mut stdout)?;

        let mut me = UI {
            runner,
            stdout,
            inbuf,
        };
        me.display_prompt()?;
        Ok(me)
    }

    pub(crate) fn cleanup(&mut self) -> anyhow::Result<()> {
        screen::exit(&mut self.stdout)
    }

    pub(crate) fn goodbye(&mut self) -> anyhow::Result<()> {
        self.stdout.write_all(GOODBYE.as_bytes())?;
        Ok(())
    }

    pub(crate) fn handle_event(&mut self, event: Event) -> anyhow::Result<()> {
        use Event::*;

        match event {
            Tick(_) => self.display_runs(),
            Terminal(evres) => {
                let event = evres?;
                self.handle_ct_event(event)
            }
            Child(event) => self.runner.handle_event(event),
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
                        self.display_runs()?;
                        self.inbuf.clear();
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
        // TODO: This is too messy with excessive clones
        let (cols, rows) = terminal::size()?;
        let childcols = cols - 1; // Cut off 1 for indicator column
        let rows = usize::from(rows);
        let mut rowtexts = vec![];

        for run in self.runner.runs().rev() {
            for (source, line) in run.format_log(childcols).rev() {
                use exoshell_runner::LogItemSource::*;

                let indicator = match source {
                    ExecutionError => '❌',
                    ChildIO => '❌',
                    ChildOut => ' ',
                    ChildErr => '⚠',
                    ChildExit => 'ⓘ',
                };
                rowtexts.push(format!("{indicator}{line}"));
            }
            let header = run.format_header(childcols);
            rowtexts.push(format!("-{header}"));
            if rowtexts.len() + 1 == rows {
                break;
            }
        }

        for (i, line) in rowtexts.into_iter().enumerate() {
            assert!(rows >= 2 + i, "rows {rows:?}, i {i:?}");
            let row = u16::try_from(rows - 2 - i).unwrap();
            self.blit_line(cols, row, &line)?;
        }
        self.display_prompt()
    }

    fn display_prompt(&mut self) -> anyhow::Result<()> {
        let inbuf = &self.inbuf;
        let (cols, rows) = terminal::size()?;
        self.blit_line(cols, rows - 1, &format!("{PROMPT}{inbuf}"))?;
        self.stdout.flush()?;
        Ok(())
    }

    fn blit_line(&mut self, cols: u16, row: u16, line: &str) -> anyhow::Result<()> {
        use crossterm::{cursor, style, QueueableCommand};

        assert!(!line.contains('\n'), "{line:?}");
        assert!(line.chars().count() <= usize::from(cols), "{line:?}");
        self.stdout
            .queue(style::SetBackgroundColor(style::Color::Reset))?
            .queue(cursor::MoveTo(0, row))?
            .queue(terminal::Clear(terminal::ClearType::CurrentLine))?
            .write_all(line.as_bytes())?;
        Ok(())
    }
}
