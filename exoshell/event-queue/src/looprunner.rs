use crate::qevent::QuitOrAppEvent;
use futures::Future;
use tokio::sync::mpsc;

/// Facilitates running an event loop over the events
#[derive(Debug)]
pub struct LoopRunner<Event>(Inner<Event>);

type Inner<Event> = mpsc::UnboundedReceiver<QuitOrAppEvent<Event>>;

impl<Event> LoopRunner<Event> {
    pub(super) fn wrap(r: Inner<Event>) -> Self {
        LoopRunner(r)
    }

    /// Run an event loop via `handle_event`
    pub async fn run<F, Fut, E>(mut self, mut handle_event: F) -> Result<(), E>
    where
        F: FnMut(Event) -> Fut,
        Fut: Future<Output = Result<(), E>>,
    {
        while let Some(ev) = self.next().await {
            handle_event(ev).await?;
        }
        Ok(())
    }

    async fn next(&mut self) -> Option<Event> {
        use QuitOrAppEvent::*;

        match self.0.recv().await.expect("EventStream closed") {
            Quit => None,
            AppEvent(ev) => Some(ev),
        }
    }
}
