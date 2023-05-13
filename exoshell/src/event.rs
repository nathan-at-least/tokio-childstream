#[derive(Debug, derive_more::From)]
#[from(forward)]
pub(crate) enum MainLoopEvent {
    Exit,
    App(AppEvent),
}

#[derive(Debug, derive_more::From)]
pub(crate) enum AppEvent {
    Terminal(std::io::Result<crossterm::event::Event>),
    Child(ChildEvent),
}

#[derive(Debug, derive_new::new)]
pub(crate) struct ChildEvent {
    #[allow(dead_code)]
    runix: usize,
    #[allow(dead_code)]
    ev: tokio_childstream::StreamItem,
}
