use crate::event::{self, EventReader};
use crate::ui::UI;

pub(crate) async fn main_loop() -> anyhow::Result<()> {
    use crate::cleanup::CleanupWith;
    use crate::Runner;

    let (reader, sender) = event::init_queue();

    sender.send_stream(crossterm::event::EventStream::default());

    let mut ui = UI::new(Runner::new(sender.clone()))?;
    main_loop_inner(reader, &mut ui)
        .await
        .cleanup_with(ui.cleanup())?;
    ui.goodbye()?;
    Ok(())
}

pub(crate) async fn main_loop_inner(mut reader: EventReader, ui: &mut UI) -> anyhow::Result<()> {
    while let Some(event) = reader.next().await {
        ui.handle_event(event)?;
    }
    Ok(())
}
