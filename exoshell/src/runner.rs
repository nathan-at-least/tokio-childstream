use crate::eventq;

#[derive(Debug)]
pub struct Runner {
    runix: usize,
    evs: eventq::Sender,
}

impl From<eventq::Sender> for Runner {
    fn from(evs: eventq::Sender) -> Self {
        Runner { runix: 0, evs }
    }
}

impl Runner {
    pub(crate) fn handle_command(&mut self, cmdtext: &str) -> anyhow::Result<()> {
        let cmdtext = cmdtext.trim();
        if cmdtext.is_empty() {
            Ok(())
        } else if cmdtext == "exit" {
            self.evs.send(crate::event::MainLoopEvent::Exit)?;
            Ok(())
        } else {
            use crate::event::ChildEvent;
            use crate::Command;
            use futures::stream::StreamExt;

            let cmd: Command = cmdtext.parse()?;
            let stream = cmd.spawn()?;
            let runix = self.runix;
            self.runix += 1;
            self.evs
                .add_producer(stream.map(move |ev| ChildEvent::new(runix, ev)));
            Ok(())
        }
    }
}
