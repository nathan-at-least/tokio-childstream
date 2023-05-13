#[derive(Debug)]
pub(crate) enum MainLoopEvent {
    #[allow(dead_code)]
    Exit,
    App(AppEvent),
}

impl<T> From<T> for MainLoopEvent
where
    AppEvent: From<T>,
{
    fn from(ev: T) -> Self {
        MainLoopEvent::App(AppEvent::from(ev))
    }
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
