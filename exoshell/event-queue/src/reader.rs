use crate::qevent::QuitOrAppEvent;
use tokio::sync::mpsc;

/// Facilitates running an event loop over the events
#[derive(Debug)]
pub struct Reader<Event>(Inner<Event>);

type Inner<Event> = mpsc::UnboundedReceiver<QuitOrAppEvent<Event>>;

impl<Event> Reader<Event> {
    pub(super) fn wrap(r: Inner<Event>) -> Self {
        Reader(r)
    }

    /// Get the next event, or [None] if every [Sender](crate::Sender) closed or in in-band quit was sent
    pub async fn next(&mut self) -> Option<Event> {
        use QuitOrAppEvent::*;

        self.0.recv().await.and_then(|ev| match ev {
            Quit => None,
            AppEvent(ev) => Some(ev),
        })
    }
}
