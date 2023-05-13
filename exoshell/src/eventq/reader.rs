use crate::event::{AppEvent, MainLoopEvent};
use tokio::sync::mpsc;

#[derive(Debug)]
pub struct Reader(mpsc::UnboundedReceiver<MainLoopEvent>);

impl Reader {
    pub(super) fn wrap(r: mpsc::UnboundedReceiver<MainLoopEvent>) -> Self {
        Reader(r)
    }

    pub(crate) async fn next(&mut self) -> Option<AppEvent> {
        use MainLoopEvent::*;

        match self.0.recv().await.expect("EventStream closed") {
            Exit => None,
            App(ev) => Some(ev),
        }
    }
}
