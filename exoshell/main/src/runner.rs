use crate::event::EventSender;

#[derive(Debug)]
pub struct Runner {
    runix: usize,
    evs: EventSender,
}

impl From<EventSender> for Runner {
    fn from(evs: EventSender) -> Self {
        Runner { runix: 0, evs }
    }
}

impl Runner {
    pub(crate) fn handle_command(&mut self, cmdtext: &str) -> anyhow::Result<()> {
        let cmdtext = cmdtext.trim();
        if cmdtext.is_empty() {
            Ok(())
        } else if cmdtext == "exit" {
            self.evs.send_quit()?;
            Ok(())
        } else {
            use crate::cmd::Command;
            use crate::event::ChildEvent;
            use futures::stream::StreamExt;

            let cmd: Command = cmdtext.parse()?;
            let stream = cmd.spawn()?;
            let runix = self.runix;
            self.runix += 1;
            self.evs
                .send_stream(stream.map(move |ev| ChildEvent::new(runix, ev)));
            Ok(())
        }
    }
}
