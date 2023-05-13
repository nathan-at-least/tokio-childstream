use crate::{AppEvent, MainLoopEvent};
use futures::stream::Stream;
use tokio::sync::mpsc;

#[derive(Debug)]
pub struct EventStream {
    r: mpsc::UnboundedReceiver<MainLoopEvent>,
    s: mpsc::UnboundedSender<MainLoopEvent>,
}

impl Default for EventStream {
    fn default() -> Self {
        let (s, r) = mpsc::unbounded_channel();
        let me = EventStream { r, s };
        me.add_producer(crossterm::event::EventStream::default());
        me
    }
}

impl EventStream {
    pub(crate) fn add_producer<S, T>(&self, mut producer: S)
    where
        S: Stream<Item = T> + Send + std::marker::Unpin + 'static,
        MainLoopEvent: From<T>,
    {
        let sender = self.s.clone();
        tokio::task::spawn(async move {
            use futures::stream::StreamExt;

            while let Some(item) = producer.next().await {
                sender.send(MainLoopEvent::from(item)).unwrap();
            }
        });
    }

    pub(crate) async fn next(&mut self) -> Option<AppEvent> {
        use MainLoopEvent::*;

        match self.r.recv().await.expect("EventStream closed") {
            Exit => None,
            App(ev) => Some(ev),
        }
    }
}
