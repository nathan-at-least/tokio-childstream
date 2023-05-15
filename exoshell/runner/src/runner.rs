use crate::{Event, Run};
use exoshell_event_queue::Sender;

#[derive(Debug, derive_new::new)]
pub struct Runner<Outer> {
    evs: Sender<Outer>,
    #[new(default)]
    runs: Vec<Run>,
}

impl<O> Runner<O>
where
    O: From<Event> + Send + std::fmt::Debug + 'static,
    Event: TryFrom<O>,
{
    pub fn handle_command(&mut self, cmdtext: &str) -> anyhow::Result<()> {
        let cmdtext = cmdtext.trim();
        if cmdtext == "exit" {
            self.evs.send_quit()?;
        } else if !cmdtext.is_empty() {
            let mut run = Run::new(cmdtext.to_string());
            if let Some(e) = self.parse_and_launch(cmdtext).err() {
                run.log_execution_error(e)
            }
            self.runs.push(run);
        }
        Ok(())
    }

    pub fn handle_event(&mut self, event: Event) -> anyhow::Result<()> {
        event.insert_into(&mut self.runs[..])
    }

    pub fn runs(&self) -> impl DoubleEndedIterator<Item = &Run> {
        self.runs.iter()
    }

    fn parse_and_launch(&self, cmdtext: &str) -> anyhow::Result<()> {
        use exoshell_command::Command;
        use futures::stream::StreamExt;

        let cmd: Command = cmdtext.parse()?;
        let stream = cmd.spawn()?;
        let runix = self.runs.len();
        self.evs
            .send_stream(stream.map(move |ev| Event::new(runix, ev)));
        Ok(())
    }
}
