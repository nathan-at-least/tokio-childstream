#[derive(Debug)]
pub(crate) enum QuitOrAppEvent<Event> {
    Quit,
    AppEvent(Event),
}

impl<E> From<E> for QuitOrAppEvent<E> {
    fn from(appevent: E) -> Self {
        QuitOrAppEvent::AppEvent(appevent)
    }
}
