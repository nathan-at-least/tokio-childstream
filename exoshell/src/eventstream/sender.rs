use crate::MainLoopEvent;
use futures::stream::Stream;
use tokio::sync::mpsc;

#[derive(Clone, Debug)]
pub struct Sender(mpsc::UnboundedSender<MainLoopEvent>);

impl Sender {
    pub(super) fn wrap(s: mpsc::UnboundedSender<MainLoopEvent>) -> Self {
        Sender(s)
    }

    pub(crate) fn send<T>(&self, event: T) -> anyhow::Result<()>
    where
        MainLoopEvent: From<T>,
    {
        self.0.send(MainLoopEvent::from(event))?;
        Ok(())
    }

    pub(crate) fn add_producer<S, T>(&self, mut producer: S)
    where
        S: Stream<Item = T> + Send + std::marker::Unpin + 'static,
        MainLoopEvent: From<T>,
    {
        let sender = self.0.clone();
        tokio::task::spawn(async move {
            use futures::stream::StreamExt;

            while let Some(item) = producer.next().await {
                sender.send(MainLoopEvent::from(item)).unwrap();
            }
        });
    }
}
