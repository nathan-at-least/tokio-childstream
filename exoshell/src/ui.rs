use crate::runner::Runner;
use crate::screen;
use crossterm::event::Event;
use std::io::{Stdout, Write};

const WELCOME: &str = "🐢 Entering the exoshell…\n";
const GOODBYE: &str = "🐢 Until next time! 👋\n";

pub struct UI {
    runner: Runner,
    stdout: Stdout,
    inbuf: String,
}

impl UI {
    pub fn new(runner: Runner) -> anyhow::Result<Self> {
        let inbuf = String::new();
        let mut stdout = crate::tty::get()?;
        stdout.write_all(WELCOME.as_bytes())?;
        screen::setup(&mut stdout)?;

        let mut me = UI {
            runner,
            stdout,
            inbuf,
        };
        me.display_prompt("$ ")?;
        Ok(me)
    }

    pub fn cleanup(&mut self) -> anyhow::Result<()> {
        screen::exit(&mut self.stdout)
    }

    pub fn goodbye(&mut self) -> anyhow::Result<()> {
        self.stdout.write_all(GOODBYE.as_bytes())?;
        Ok(())
    }

    pub fn handle_event(&mut self, ev: Event) -> anyhow::Result<()> {
        use crossterm::event::{Event::Key, KeyEvent, KeyEventKind};

        match ev {
            Key(KeyEvent {
                code,
                kind: KeyEventKind::Press,
                ..
            }) => {
                use crossterm::event::KeyCode::{Char, Enter};

                match code {
                    Enter => self.runner.handle_command(&self.inbuf),
                    Char(c) => {
                        self.inbuf.push(c);

                        // Display it on screen:
                        let mut bytes = [0u8; 4];
                        c.encode_utf8(&mut bytes);
                        self.stdout.write_all(&bytes[..c.len_utf8()])?;
                        self.stdout.flush()?;
                        Ok(())
                    }
                    _ => Ok(()),
                }
            }
            _ => Ok(()),
        }
    }

    fn display_prompt(&mut self, prompt: &str) -> anyhow::Result<()> {
        use crossterm::{cursor, style, terminal, QueueableCommand};

        let (_, rows) = terminal::size()?;
        self.stdout
            .queue(style::SetBackgroundColor(style::Color::Reset))?
            .queue(cursor::MoveTo(0, rows - 1))?
            .write_all(prompt.as_bytes())?;
        self.stdout.flush()?;
        Ok(())
    }
}
