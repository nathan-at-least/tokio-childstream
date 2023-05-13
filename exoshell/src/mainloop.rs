use crate::{eventq, UI};

pub(crate) async fn main_loop() -> anyhow::Result<()> {
    use crate::cleanup::CleanupWith;
    use crate::Runner;

    let (r, s) = eventq::init();

    let mut ui = UI::new(Runner::from(s.clone()))?;
    main_loop_inner(r, &mut ui)
        .await
        .cleanup_with(ui.cleanup())?;
    ui.goodbye()?;
    Ok(())
}

pub(crate) async fn main_loop_inner(mut evr: eventq::Reader, ui: &mut UI) -> anyhow::Result<()> {
    use crate::AppEvent::*;

    while let Some(event) = evr.next().await {
        match event {
            Terminal(evres) => {
                let ev = evres?;
                ui.handle_event(ev)?;
            }
            other => todo!("{other:#?}"),
        }
    }
    Ok(())
}
