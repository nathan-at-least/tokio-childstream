use crate::{EventStream, UI};

pub(crate) async fn main_loop() -> anyhow::Result<()> {
    use crate::cleanup::CleanupWith;

    let mut ui = UI::new()?;
    main_loop_inner(&mut ui).await.cleanup_with(ui.cleanup())?;
    ui.goodbye()?;
    Ok(())
}

pub(crate) async fn main_loop_inner(ui: &mut UI) -> anyhow::Result<()> {
    use crate::AppEvent::*;

    let mut runix = 0;
    let mut events = EventStream::default();
    while let Some(event) = events.next().await {
        match event {
            Terminal(evres) => {
                let ev = evres?;
                if let Some(cmdtext) = ui.handle_event(ev)? {
                    use crate::event::ChildEvent;
                    use crate::Command;
                    use futures::stream::StreamExt;

                    let cmd: Command = cmdtext.parse()?;
                    let childstream = cmd.spawn()?;
                    events.add_producer(
                        childstream.map(move |childevent| ChildEvent::new(runix, childevent)),
                    );
                    runix += 1;
                }
            }
            other => todo!("{other:#?}"),
        }
    }
    Ok(())
}
